use std::fmt;

#[derive(Clone, Debug)]
pub struct NetworkPeer {
    pub id: String,
    pub addr: String,
}

impl fmt::Display for NetworkPeer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.id, self.addr)
    }
}
