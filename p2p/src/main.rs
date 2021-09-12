use std::{collections::HashSet, env::args, io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, sync::Mutex, thread::{self, JoinHandle, Thread, spawn}, time::Duration};

use peers_file_handler::get_remote_peers;
use remote_peer::RemotePeer;
use dashmap::DashSet;
use std::iter::FromIterator;
use std::sync::Arc;
use std::sync::atomic::Ordering::AcqRel;

mod remote_peer;
mod peers_file_handler;

struct AppContext {
    host_address: String,
    remotes_set: DashSet<RemotePeer>
}

fn handle_connection(con: (TcpStream, SocketAddr)) -> Result<(), std::io::Error> {
    let mut back = TcpStream::connect(con.1)?;
    let mut buffer = String::new();
    back.read_to_string(&mut buffer)?;
    println!("Message from: {} -> {}", con.1, buffer.trim());
    back.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}

fn set_up_listener(ctx: &AppContext) -> std::io::Result<()> {
    println!("binding address {}", ctx.host_address);
    let listener = TcpListener::bind(&ctx.host_address)?;
    loop {
        if let Ok(con) = listener.accept() {
            println!("Peer connected {:?}", con.1);
            handle_connection(con)?;
        }
    }
}

fn connect_to_remote_peer(ctx: Arc<AppContext>, self_address: String, peer_address: String) -> Result<(), std::io::Error> {
    let mut tcp_stream = TcpStream::connect(&peer_address)?;
    let connected_peer_entry = RemotePeer::from_address(&peer_address).connected();
    ctx.remotes_set.insert(connected_peer_entry);
    let hello_string = format!("Hello from {}\n", self_address);
    tcp_stream.write_all(hello_string.as_bytes())?;
    Ok(())
}

fn connect_with_remote_peers(ctx: Arc<AppContext>) -> Vec<JoinHandle<()>> {
    ctx.remotes_set
        .iter()
        .map(|remote_peer| spawn_remote_peer_connection_thread(Arc::<AppContext>::clone(&ctx), remote_peer.get_address()))
        .collect()
}

fn spawn_remote_peer_connection_thread(ctx: Arc<AppContext>, remote_peer_address: String) -> JoinHandle<()> {
    let host_address = ctx.host_address.clone();
    let remote_peer_address = remote_peer_address;
    let handle = spawn(move|| {
        println!("{} Connection thread spawned", remote_peer_address);
        connect_to_remote_peer(ctx, host_address, remote_peer_address).unwrap();
    });
    return handle;
}

fn close_remote_peers_connections(threads: Vec<JoinHandle<()>>) {
    //TODO send kill connection message
    threads
        .into_iter()
        .for_each(|handle| handle.join().expect("Couldn't join thread!"));
}

fn get_command_line_opt_value(opt_char: char) -> Option<String> {
    let opt_str = format!("-{}", opt_char);
    args().skip(1)
        .fold(None, |mut acc, x| {
            println!("{}", x);
            let acc_tmp = acc.clone();
            if x == opt_str {
                acc = Some(opt_str.clone());
                return acc
            }
            if let Some(opt) = acc{
                if opt == opt_str {
                    acc = Some(x);
                    return acc
                }
                acc_tmp
            } else {
                acc
            }
        })
}

fn get_host_address_from_args() -> Option<String> {
    return get_command_line_opt_value('H')
}

fn get_peers_file_from_args() -> Option<String> {
    return get_command_line_opt_value('f')
}

fn display_usage_prompt() {
    println!("Usage: app -H <host>::<port> <peer host>::<port> -f <peers_file_path>")
}

fn main(){
    let host_address = match get_host_address_from_args() {
        Some(address) => address,
        None => {
            display_usage_prompt();
            return
        }
    };

    let peers_file = match get_peers_file_from_args() {
        Some(address) => address,
        None => {
            display_usage_prompt();
            return
        }
    };

    println!("{}\n{}", host_address, peers_file);

    let peers_list = get_remote_peers(peers_file);
    let mut remotes_set = HashSet::<RemotePeer>::new();
    for peer in peers_list.into_iter() {
        remotes_set.insert(peer);
    }

    let ctx = AppContext{
        host_address,
        remotes_set: DashSet::from_iter(remotes_set)
    };

    let ctx_ref = Arc::new(ctx);
    connect_with_remote_peers(Arc::clone(&ctx_ref));

    if let Err(err) = set_up_listener(&Arc::clone(&ctx_ref)) {
        println!("An error has occured {:?}", err)
    }
}