use std::{ collections::HashMap, env::args, io::{BufRead, Read, Seek, Write}, net::{SocketAddr, TcpListener, TcpStream}, thread::{spawn, JoinHandle}};

struct HostContext {
    host_address: String,
    connected_peers: HashMap<String, RemotePeer>
}

#[derive(Debug)]
enum RemotePeerState {
    NOT_CONNECTED,
    CONNECTED
}

#[derive(Debug)]
struct RemotePeer {
    address: String,
    state: RemotePeerState
}

#[derive(Debug)]
struct ThreadContext {
    remote_peer: RemotePeer,
    join_handle: Option<JoinHandle<()>>,
}






fn main() -> std::io::Result<()> {
    let address = args()
        .skip(1)
        .next()
        .expect("Usage: app <host>::<port> <peer host>::<port>");
    let ctx = HostContext {
        host_address: address.clone(),
        peers
    };
    
    let remote_connections_threads = connect_with_remote_peers(&ctx, peers_list);
    set_up_listener(&ctx).unwrap();
    close_remote_peers_connections(remote_connections_threads);
    Ok(())
}
