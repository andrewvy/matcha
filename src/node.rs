use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use futures::sync::mpsc;
use futures::{Future, Sink, Stream};

use tokio_io::{AsyncRead};
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;
use tokio_core::net::{TcpStream, TcpListener};

use failure::Error;

use codec::MessageCodec;
use matcha_pb::{Message};

type ServerChannel = mpsc::UnboundedSender<ServerMessage>;
type ServerReceiver = mpsc::UnboundedReceiver<ServerMessage>;
type PeerChannel = mpsc::UnboundedSender<Message>;

#[derive(Debug)]
struct Node {
    pub addr: SocketAddr,
    pub data: Rc<RefCell<NodeData>>,
}

impl Node {
    pub fn new(addr: SocketAddr) -> Node {
        let node_data = NodeData {
            peers: HashMap::new()
        };

        Node {
            addr: addr,
            data: Rc::new(RefCell::new(node_data))
        }
    }
}

#[derive(Debug)]
struct NodeData {
    pub peers: HashMap<SocketAddr, PeerChannel>
}

#[derive(Debug)]
pub enum ServerMessage {
    PeerJoined(SocketAddr, PeerChannel),
    PeerLeft(SocketAddr),
}

fn connect_to_peer(handle: Handle, peer_addr: SocketAddr, tx_channel: ServerChannel) -> Box<Future<Item = (), Error = io::Error>> {
    let client = TcpStream::connect(&peer_addr, &handle).and_then(move |conn| {
        let (sink, stream) = conn.framed(MessageCodec).split();
        let (peer_tx, peer_rx) = mpsc::unbounded::<Message>();
        let peer_to_server_channel = tx_channel.clone();
        let inner_handle = handle.clone();

        peer_to_server_channel
            .unbounded_send(ServerMessage::PeerJoined(peer_addr, peer_tx.clone()))
            .expect("Could not send to server");

        let reader = stream
            .for_each(move |msg| handle_message(peer_addr, msg, inner_handle.clone(), peer_tx.clone(), peer_to_server_channel.clone()))
            .map_err(|_| format_err!(""))
            .then(move |_| {
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

fn serve(handle: Handle, addr: &SocketAddr, tx_channel: ServerChannel) -> Box<Future<Item = (), Error = io::Error>> {
    let listener = TcpListener::bind(addr, &handle).unwrap();

    println!("Listening on: {}", addr);

    let server =
        listener
            .incoming()
            .for_each(move |(conn, peer_addr)| {
                let (sink, stream) = conn.framed(MessageCodec).split();
                let (peer_tx, peer_rx) = mpsc::unbounded::<Message>();
                let peer_to_server_channel = tx_channel.clone();
                let inner_handle = handle.clone();

                peer_to_server_channel
                    .unbounded_send(ServerMessage::PeerJoined(peer_addr, peer_tx.clone()))
                    .expect("Could not send to server");

                let shutdown_channel = tx_channel.clone();

                let reader = stream
                    .for_each(move |msg| handle_message(peer_addr, msg, inner_handle.clone(), peer_tx.clone(), peer_to_server_channel.clone()))
                    .map_err(|_| format_err!(""))
                    .then(move |_| {
                        shutdown_channel
                            .unbounded_send(ServerMessage::PeerLeft(peer_addr))
                            .expect("Could not send to server");

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

fn handle_message(_peer_addr: SocketAddr, _message: Message, _handle: Handle, _peer_tx: PeerChannel, _server_tx: ServerChannel) -> Result<(), Error> {
    Ok(())
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

    let (server_channel, receiver): (ServerChannel, ServerReceiver) = mpsc::unbounded();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let node = Node::new(parsed_server_addr);

    let server = serve(handle.clone(), &parsed_server_addr, server_channel.clone());

    let inner_handle = handle.clone();

    formatted_bootstrap_nodes.into_iter().for_each(move |peer_addr| {
        inner_handle.spawn(
            connect_to_peer(inner_handle.clone(), peer_addr, server_channel.clone()).then(|_| Ok(()))
        )
    });

    handle.clone().spawn(
        receiver.for_each(move |msg| {
            match msg {
                ServerMessage::PeerJoined(addr, channel) => {
                    node.data.borrow_mut().peers.insert(addr, channel);
                    println!("peers: {:?}", node.data.borrow().peers);
                }
                ServerMessage::PeerLeft(addr) => {
                    node.data.borrow_mut().peers.remove(&addr);
                    println!("peers: {:?}", node.data.borrow().peers);
                }
            }

            Ok(())
        }).and_then(|_| Ok(()))
    );

    core.run(server).unwrap();
}
