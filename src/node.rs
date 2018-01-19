use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use futures::sync::mpsc;
use futures::{Future, Sink, Stream};

use tokio_io::{AsyncRead};
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpStream, TcpListener};

use failure::Error;

use codec::MessageCodec;
use matcha_pb::{self, Message, Request, Response};

type PeerChannel = mpsc::UnboundedSender<Message>;

type RefNodeData = Rc<RefCell<NodeData>>;

#[derive(Debug, Clone)]
struct Node {
    pub data: Rc<RefCell<NodeData>>,
}

impl Node {
    pub fn new(addr: SocketAddr) -> Node {
        let node_data = NodeData {
            addr: addr,
            peers: HashMap::new()
        };

        Node {
            data: Rc::new(RefCell::new(node_data))
        }
    }
}

#[derive(Debug)]
struct NodeData {
    pub addr: SocketAddr,
    pub peers: HashMap<SocketAddr, PeerChannel>
}

fn connect_to_peer(node_data: RefNodeData, handle: Handle, peer_addr: SocketAddr) -> Box<Future<Item = (), Error = io::Error>> {
    let client = TcpStream::connect(&peer_addr, &handle).and_then(move |conn| {
        let (sink, stream) = conn.framed(MessageCodec).split();
        let (peer_tx, peer_rx) = mpsc::unbounded::<Message>();
        let inner_handle = handle.clone();
        let inner_node_data = node_data.clone();

        add_peer(node_data.clone(), peer_addr, peer_tx.clone());

        let mut peer = matcha_pb::Peer::new();
        peer.set_addr(node_data.borrow().addr.to_string());

        let mut peer_list = matcha_pb::PeerList::new();
        peer_list.mut_peers().push(peer);

        let mut peer_list_request = matcha_pb::PeerListRequest::new();
        peer_list_request.set_peer_list(peer_list);

        let mut request = Request::new();
        request.set_field_type(matcha_pb::Request_Type::PEER_LIST_REQUEST);
        request.set_peer_list_request(peer_list_request);

        let mut message = Message::new();
        message.set_field_type(matcha_pb::Message_Type::REQUEST);
        message.set_request(request);

        peer_tx.clone().unbounded_send(message).expect("could not send peerlist");

        let reader = stream
            .for_each(move |msg| handle_message(node_data.clone(), peer_addr, msg, inner_handle.clone(), peer_tx.clone()))
            .map_err(|_| format_err!(""))
            .then(move |_| {
                remove_peer(inner_node_data.clone(), peer_addr);
                Ok(())
            });

        let writer = sink.send_all(peer_rx.map_err(|_| format_err!("Error in sending"))).then(|_| {
            Ok(())
        });
        
        handle.spawn(reader);
        handle.spawn(writer);

        Ok(())
    });

    Box::new(client)
}

fn serve(node_data: RefNodeData, handle: Handle, addr: &SocketAddr) -> Box<Future<Item = (), Error = io::Error>> {
    let listener = TcpListener::bind(addr, &handle).unwrap();

    println!("Listening on: {}", addr);

    let server =
        listener
            .incoming()
            .for_each(move |(conn, peer_addr)| {
                let (sink, stream) = conn.framed(MessageCodec).split();
                let (peer_tx, peer_rx) = mpsc::unbounded::<Message>();
                let inner_handle = handle.clone();

                let handler_node_data = node_data.clone();
                let inner_node_data = node_data.clone();

                add_peer(node_data.clone(), peer_addr, peer_tx.clone());

                let reader = stream
                    .for_each(move |msg| handle_message(handler_node_data.clone(), peer_addr, msg, inner_handle.clone(), peer_tx.clone()))
                    .map_err(|_| format_err!(""))
                    .then(move |_| {
                        remove_peer(inner_node_data.clone(), peer_addr);
                        Ok(())
                    });

                let writer = sink.send_all(peer_rx.map_err(|_| format_err!("Error in sending"))).then(|_| {
                    Ok(())
                });
                
                handle.spawn(reader);
                handle.spawn(writer);

                Ok(())
            });

    Box::new(server)
}

fn add_peer(node_data: RefNodeData, peer_addr: SocketAddr, peer_tx: PeerChannel) {
    println!("adding peer to peerlist: {:?}", peer_addr);
    node_data.borrow_mut().peers.insert(peer_addr, peer_tx.clone());
}

fn remove_peer(node_data: RefNodeData, peer_addr: SocketAddr) {
    println!("removing peer from peerlist: {:?}", peer_addr);
    node_data.borrow_mut().peers.remove(&peer_addr);
}

fn is_already_connected_to_peer(node_data: RefNodeData, peer_addr: SocketAddr) -> bool {
    node_data.borrow().peers.contains_key(&peer_addr)
}

fn handle_message(node_data: RefNodeData, _peer_addr: SocketAddr, message: Message, handle: Handle, peer_tx: PeerChannel) -> Result<(), Error> {
    match message.get_field_type() {
        matcha_pb::Message_Type::REQUEST => handle_request(node_data, message.get_request(), peer_tx, handle),
        matcha_pb::Message_Type::RESPONSE => handle_response(node_data, message.get_response(), peer_tx, handle),
    }
}

fn handle_request(node_data: RefNodeData, request: &Request, _peer_tx: PeerChannel, handle: Handle) -> Result<(), Error> {
    match request.get_field_type() {
        matcha_pb::Request_Type::PING_REQUEST => {
            // node_data.borrow_mut().handle_ping_request(request.get_ping_request(), channel)
            Ok(())
        }
        matcha_pb::Request_Type::PEER_LIST_REQUEST => {
            let peer_list_request = request.get_peer_list_request();
            let peer_list = peer_list_request.get_peer_list();

            for peer in peer_list.get_peers() {
                let inner_node_data = node_data.clone();
                let inner_handle = handle.clone();
                let addr = String::from(peer.get_addr()).parse()?;

                if !is_already_connected_to_peer(inner_node_data.clone(), addr) || addr == node_data.borrow().addr {
                    println!("adding new peer: {:?}", addr);

                    inner_handle.spawn(
                        connect_to_peer(inner_node_data.clone(), inner_handle.clone(), addr).then(|_| Ok(()))
                    );
                }
            }

            Ok(())
        }
    }
}

fn handle_response(_node_data: Rc<RefCell<NodeData>>, response: &Response, _peer_tx: PeerChannel, _handle: Handle) -> Result<(), Error> {
    match response.get_field_type() {
        matcha_pb::Response_Type::PING_RESPONSE => {
            // node_data.borrow_mut().handle_ping_response(response.get_ping_response(), channel)
            Ok(())
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


pub fn boot(server_addr: String, bootstrap_nodes: Vec<&str>) {
    let parsed_server_addr: SocketAddr = server_addr
        .parse()
        .expect("No valid port specified");

    let formatted_bootstrap_nodes: Vec<SocketAddr> = bootstrap_nodes
        .into_iter()
        .map(|addr| {
            String::from(addr).parse().expect("Malformed bootstrap node address")
        })
        .collect();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let node = Node::new(parsed_server_addr);

    let server = serve(node.data.clone(), handle.clone(), &parsed_server_addr);
    let inner_handle = handle.clone();

    for addr in formatted_bootstrap_nodes {
        let inner_node_data = node.data.clone();

        if !is_already_connected_to_peer(inner_node_data.clone(), addr) {
            inner_handle.spawn(
                connect_to_peer(inner_node_data, inner_handle.clone(), addr).then(|_| Ok(()))
            );
        }
    }

    core.run(server).unwrap();
}
