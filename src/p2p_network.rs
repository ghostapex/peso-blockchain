use libp2p::{core::multiaddr::Protocol, Multiaddr};
use std::str::FromStr;

pub struct P2PNetwork {
    pub local_peer_id: String,
    pub peers: Vec<String>,
}

impl P2PNetwork {
    pub fn new(local_peer_id: String) -> Self {
        Self {
            local_peer_id,
            peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, peer: String) {
        if !self.peers.contains(&peer) {
            self.peers.push(peer);
        }
    }

    pub fn broadcast_block(&self, block_hash: &str) {
        for peer in &self.peers {
            println!("Broadcasting block {} to peer {}", block_hash, peer);
        }
    }

    pub fn broadcast_tx(&self, tx_hash: &str) {
        for peer in &self.peers {
            println!("Broadcasting transaction {} to peer {}", tx_hash, peer);
        }
    }
}
