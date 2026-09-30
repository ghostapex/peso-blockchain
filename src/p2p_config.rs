use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addr: SocketAddr,
    pub peer_id: String,
    pub bootstrap_peers: Vec<String>,
    pub gossip_topic: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:9000".parse().unwrap(),
            peer_id: "peso-node-1".to_string(),
            bootstrap_peers: vec![
                "/ip4/127.0.0.1/tcp/9001".to_string(),
                "/ip4/127.0.0.1/tcp/9002".to_string(),
            ],
            gossip_topic: "peso-blocks".to_string(),
        }
    }
}
