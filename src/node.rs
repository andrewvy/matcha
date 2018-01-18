use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use futures::{Future, Stream, Sink};
use futures::sync::mpsc;

use tokio_io::{AsyncRead};
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;
use tokio_core::net::{TcpStream, TcpListener};
use tokio_timer::Timer;
use uuid::Uuid;
use rand::{thread_rng, Rng, ThreadRng};

use failure::Error;

use codec::MessageCodec;
use matcha_pb::{self, Message, Request, Response};

type Channel = mpsc::UnboundedSender<Message>;

#[derive(Clone)]
pub struct Node {
    node_data: Rc<RefCell<NodeData>>,
    pub timer: Timer,
}

impl Node {
    pub fn new(addr: SocketAddr) -> Node {
        let node = NodeData {
            id: Uuid::new_v4(),
            addr: addr,
            peers: HashMap::new(),
            rng: Rc::new(RefCell::new(thread_rng())),
        };

        Node {
            node_data: Rc::new(RefCell::new(node)),
            timer: Timer::default()
        }
    }

    pub fn run<I: Iterator<Item=SocketAddr>>(&self, handle: Handle, bootstrap_addrs: I) -> Box<Future<Item=(), Error=io::Error>> {
        let node_data = self.node_data.clone();

        // Start our node.
        let server = Node::serve(node_data.clone(), handle.clone());

        // Connect to any specified bootstrap addresses.
        for addr in bootstrap_addrs {
            let node_data = node_data.clone();
            Node::start_client(node_data, handle.clone(), addr);
        }

        // Every 5 secs, send our peerlist to another random peer.
        handle.spawn(self.gossip_peers(Duration::from_secs(5)).then(|_| {
            Ok(())
        }));

        server
    }

    fn start_client(node_data: Rc<RefCell<NodeData>>, handle: Handle, addr: SocketAddr) {
        handle.spawn(Node::start_client_actual(node_data, handle.clone(), &addr).then(move |x| {
            println!("client {} done {:?}", addr, x);

            Ok(())
        }));
    }

    fn start_client_actual(node_data: Rc<RefCell<NodeData>>, handle: Handle, addr: &SocketAddr) -> Box<Future<Item=(), Error=io::Error>> {
        println!("starting client {}", addr);
        let client = TcpStream::connect(&addr, &handle).and_then(move |socket| {
            println!("connected... local: {:?}, peer {:?}", socket.local_addr(), socket.peer_addr());

            let (sink, stream) = socket.framed(MessageCodec).split();
            let (channel, rx) = mpsc::unbounded();

            let node_data1 = node_data.clone();
            let channel1 = channel.clone();
            let handle1 = handle.clone();
            let read = stream.for_each(move |msg| {
                Node::process(node_data1.clone(), msg, channel1.clone(), handle1.clone())
            });

            handle.spawn(read.then(|_| Ok(())));

            let node_data2 = node_data.clone();
            let channel2 = channel.clone();

            let mut peer = matcha_pb::Peer::new();
            peer.set_uuid(node_data2.borrow().id.to_string());
            peer.set_addr(node_data2.borrow().addr.to_string());

            let mut ping_request = matcha_pb::PingRequest::new();
            ping_request.set_peer(peer);

            let mut request = Request::new();
            request.set_field_type(matcha_pb::Request_Type::PING_REQUEST);
            request.set_ping_request(ping_request);

            let mut message = Message::new();
            message.set_field_type(matcha_pb::Message_Type::REQUEST);
            message.set_request(request);

            mpsc::UnboundedSender::unbounded_send(&channel2, message)
                .expect("channel failed");

            let write = sink.send_all(rx.map_err(|()| {
                io::Error::new(io::ErrorKind::Other, "rx shouldn't have an error")
            }));

            handle.spawn(write.then(|_| Ok(())));

            Ok(())
        });

        return Box::new(client);
    }

    fn serve(node_data: Rc<RefCell<NodeData>>, handle: Handle) -> Box<Future<Item=(), Error=io::Error>> {
        let socket = TcpListener::bind(&node_data.borrow().addr, &handle).unwrap();

        println!("listening on {}", node_data.borrow().addr);

        let srv = socket.incoming().for_each(move |(tcpstream, _)| {
            let (sink, stream) = tcpstream.framed(MessageCodec).split();
            let (channel, rx) = mpsc::unbounded();

            let node_data1= node_data.clone();
            let channel1 = channel.clone();
            let handle1 = handle.clone();
            let read = stream.for_each(move |msg| {
                Node::process(node_data1.clone(), msg, channel1.clone(), handle1.clone())
            });

            handle.spawn(read.then(|_| Ok(())));

            let write = sink.send_all(rx.map_err(|()| {
                io::Error::new(io::ErrorKind::Other, "rx shouldn't have an error")
            }));

            handle.spawn(write.then(|_| Ok(())));

            Ok(())
        });

        Box::new(srv)
    }

    fn process(node_data: Rc<RefCell<NodeData>>, message: Message, channel: Channel, handle: Handle) -> Result<(), Error> {
        println!("processing message: {:?}", message);

        match message.get_field_type() {
            matcha_pb::Message_Type::REQUEST => {
                let request = message.get_request();

                match request.get_field_type() {
                    matcha_pb::Request_Type::PING_REQUEST => {
                        node_data.borrow_mut().handle_ping_request(request.get_ping_request(), channel)
                    }
                    matcha_pb::Request_Type::PEER_LIST_REQUEST => {
                        let peer_list_request = request.get_peer_list_request();
                        let peer_list = peer_list_request.get_peer_list();

                        for peer in peer_list.get_peers() {
                            let uuid = Uuid::parse_str(peer.get_uuid())?;
                            let addr = String::from(peer.get_addr()).parse()?;
                            let current_peer_list = &node_data.borrow().peers;

                            if !current_peer_list.contains_key(&uuid) && uuid != node_data.borrow().id {
                                println!("adding new node! {:?}", (uuid, addr));
                                Node::start_client(node_data.clone(), handle.clone(), addr);
                            }
                        }

                        Ok(())
                    }
                }
            }
            matcha_pb::Message_Type::RESPONSE => {
                let response = message.get_response();

                match response.get_field_type() {
                    matcha_pb::Response_Type::PING_RESPONSE => {
                        node_data.borrow_mut().handle_ping_response(response.get_ping_response(), channel)
                    }
                    matcha_pb::Response_Type::PEER_LIST_RESPONSE => {
                        // node_data.borrow_mut().handle_peer_list(request.get_peer_list_response(), channel),
                        Ok(())
                    }
                    matcha_pb::Response_Type::DESCRIPTION_ONLY => {
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn gossip_peers(&self, duration: Duration) -> Box<Future<Item=(), Error=io::Error>> {
        let node_data = self.node_data.clone();
        let future = self.timer.interval(duration).for_each(move |_| {
            let node_data1 = node_data.clone();
            let peers = node_data1.borrow().peers
                .iter()
                .map(|(uuid, &(_, addr))| {
                    let mut peer = matcha_pb::Peer::new();
                    peer.set_uuid(uuid.clone().to_string());
                    peer.set_addr(addr.clone().to_string());
                    peer
                })
                .collect();

            let mut peer_list = matcha_pb::PeerList::new();
            peer_list.set_peers(peers);

            let mut peer_list_request = matcha_pb::PeerListRequest::new();
            peer_list_request.set_peer_list(peer_list);

            let mut request = Request::new();
            request.set_field_type(matcha_pb::Request_Type::PEER_LIST_REQUEST);
            request.set_peer_list_request(peer_list_request);

            let mut message = Message::new();
            message.set_field_type(matcha_pb::Message_Type::REQUEST);
            message.set_request(request);

            Ok(node_data.borrow().send_random(message))
        });

        Box::new(future.map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e)
        }))
    }
}


struct NodeData {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub peers: HashMap<Uuid, (Channel, SocketAddr)>,
    rng: Rc<RefCell<ThreadRng>>,
}

impl NodeData {
    pub fn send_random(&self, message: Message) {
        let number_of_peers = self.peers.len();

        if number_of_peers == 0 {
            return;
        } else {
            loop {
                for peer in self.peers.values() {
                    let channel = &peer.0;

                    if self.rng.borrow_mut().gen_range(0, number_of_peers) == 0 {
                        channel.unbounded_send(message).expect("channel send failed");
                        return;
                    }
                }
            }
        }
    }

    fn handle_ping_request(&mut self, ping_request: &matcha_pb::PingRequest, channel: Channel) -> Result<(), Error> {
        println!("{:?}", ping_request);

        let uuid = Uuid::parse_str(ping_request.get_peer().get_uuid())?;
        let addr = String::from(ping_request.get_peer().get_addr()).parse()?;

        match self.peers.get(&uuid) {
            Some(_) => {
                // @TODO(vy): Drop this connection, as we already have a connection!
                Ok(())
            }
            None => {
                println!("adding new node! {:?}", uuid);

                let channel2 = channel.clone();
                self.peers.insert(uuid, (channel, addr));

                let mut peer = matcha_pb::Peer::new();
                peer.set_uuid(self.id.to_string());
                peer.set_addr(self.addr.to_string());

                let mut ping_response = matcha_pb::PingResponse::new();
                ping_response.set_peer(peer);

                let mut response = Response::new();
                response.set_status(matcha_pb::Response_Status::ACK);
                response.set_field_type(matcha_pb::Response_Type::PING_RESPONSE);
                response.set_ping_response(ping_response);

                let mut message = Message::new();
                message.set_field_type(matcha_pb::Message_Type::RESPONSE);
                message.set_response(response);

                mpsc::UnboundedSender::unbounded_send(&channel2, message)
                    .map_err(|e| Error::from(e))
            }
        }
    }

    fn handle_ping_response(&mut self, ping_response: &matcha_pb::PingResponse, channel: Channel) -> Result<(), Error> {
        let uuid = Uuid::parse_str(ping_response.get_peer().get_uuid())?;
        let addr = String::from(ping_response.get_peer().get_addr()).parse()?;

        match self.peers.get(&uuid) {
            Some(_) => {
                // @TODO(vy): Drop this connection, as we already have a connection!
            }
            None => {
                println!("adding new node! {:?}", uuid);
                self.peers.insert(uuid, (channel, addr));
            }
        }
        Ok(())
    }
}

pub fn boot(server_addr: String, bootstrap_nodes: Vec<&str>) {
    let parsed_server_addr: SocketAddr = server_addr.parse().expect("No valid port specified");
    let formatted_bootstrap_nodes = bootstrap_nodes
        .into_iter()
        .map(|addr| {
            String::from(addr).parse().expect("Malformed bootstrap node address")
        });

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let node = Node::new(parsed_server_addr);
    let server = node.run(handle.clone(), formatted_bootstrap_nodes);

    core.run(server).unwrap();
}
