#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum RemotePeerState {
    NOT_CONNECTED,
    CONNECTED
}

#[derive(Debug, Clone)]
pub struct RemotePeer {
    address: String,
    state: RemotePeerState
}

impl std::hash::Hash for RemotePeer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl PartialEq for RemotePeer {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for RemotePeer {}

impl RemotePeer {
    pub fn from_address(address: &String) -> Self {
        Self {
            address: address.clone(),
            state: RemotePeerState::NOT_CONNECTED
        }
    }

    pub fn connected(self) -> Self {
        Self{
            state: RemotePeerState::CONNECTED,
                ..self
        }
    }

    pub fn get_address(&self) -> String{
        self.address.clone()
    }
}