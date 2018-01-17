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
use protocol::Message;

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

            mpsc::UnboundedSender::unbounded_send(&channel2, Message::Ping((node_data2.borrow().id, node_data2.borrow().addr.clone())))
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

        match message {
            Message::Ping(payload) => node_data.borrow_mut().handle_ping(payload, channel),
            Message::Pong(payload) => node_data.borrow_mut().handle_pong(payload, channel),
            Message::PeerList(payload) => {
                for (id, addr) in payload {
                    let current_peer_list = &node_data.borrow().peers;

                    if !current_peer_list.contains_key(&id) && id != node_data.borrow().id {
                        println!("adding new node! {:?}", (id, addr));
                        Node::start_client(node_data.clone(), handle.clone(), addr);
                    }
                }
                Ok(())
            },
        }
    }

    pub fn gossip_peers(&self, duration: Duration) -> Box<Future<Item=(), Error=io::Error>> {
        let node_data = self.node_data.clone();
        let future = self.timer.interval(duration).for_each(move |_| {
            let node_data1 = node_data.clone();
            let message = node_data1.borrow().peers
                .iter()
                .map(|(k, v)| (k.clone(), v.1.clone()))
                .collect();

            Ok(node_data.borrow().send_random(Message::PeerList(message)))
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

    fn handle_ping(&mut self, message: (Uuid, SocketAddr), channel: Channel) -> Result<(), Error> {
        match self.peers.get(&message.0) {
            Some(_) => {
                // @TODO(vy): Drop this connection, as we already have a connection!
                Ok(())
            }
            None => {
                println!("adding new node! {:?}", message);

                let channel2 = channel.clone();
                self.peers.insert(message.0, (channel, message.1));

                mpsc::UnboundedSender::unbounded_send(&channel2, Message::Pong((self.id, self.addr)))
                    .map_err(|e| Error::from(e))
            }
        }
    }

    fn handle_pong(&mut self, message: (Uuid, SocketAddr), channel: Channel) -> Result<(), Error> {
        match self.peers.get(&message.0) {
            Some(_) => {
                // @TODO(vy): Drop this connection, as we already have a connection!
            }
            None => {
                println!("adding new node! {:?}", message);
                self.peers.insert(message.0, (channel, message.1));
            }
        }
        Ok(())
    }
}

pub fn boot(server_addr: String, bootstrap_nodes: Vec<&str>) {
    let parsed_server_addr: SocketAddr = server_addr.parse().unwrap();
    let formatted_bootstrap_nodes = bootstrap_nodes
        .into_iter()
        .map(|addr| {
            String::from(addr).parse().unwrap()
        });

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let node = Node::new(parsed_server_addr);
    let server = node.run(handle.clone(), formatted_bootstrap_nodes);

    core.run(server).unwrap();
}
