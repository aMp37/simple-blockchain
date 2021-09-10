use std::io::{BufRead, Seek};

use crate::remote_peer::RemotePeer;

const PEERS_FILE_NAME: &str = "peers.txt";

fn get_peers_file_handle() -> std::fs::File {
    let peers_file_path = format!("{}/{}", env!("PWD"), PEERS_FILE_NAME);

    return if let Ok(file_handle) = std::fs::File::open(&peers_file_path) {
        file_handle
    } else {
        std::fs::File::create(&peers_file_path).expect("cannot create peers.txt file")
    };
}

fn check_file_is_empty(file: &std::fs::File) -> bool {
    let mut file_reader = std::io::BufReader::new(file);
    let len = file_reader.seek(std::io::SeekFrom::End(0)).unwrap();
    return len == 0;
}

fn read_peers_addresses_from_file(file: &std::fs::File) -> Vec<String> {
    let mut file_reader = std::io::BufReader::new(file);
    file_reader.seek(std::io::SeekFrom::Start(0)).unwrap();
    file_reader
        .lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}

pub fn get_remote_peers() -> Vec<RemotePeer> {
    let peers_file_handle = get_peers_file_handle();
    if check_file_is_empty(&peers_file_handle) {
        //TODO handle peers file empty
        Vec::<RemotePeer>::new()
    } else {
        let peers = read_peers_addresses_from_file(&peers_file_handle);
        peers.iter().map(RemotePeer::from_address).collect()
    }
}