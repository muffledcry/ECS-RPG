use std::net::{Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct ConnectionInfo {
    pub session_id: Uuid,
    pub remote_addr: RemoteAddr,
}

#[derive(Debug, Clone)]
pub struct RemoteAddr(SocketAddr);

impl ConnectionInfo {
    pub fn new(session_id: Uuid, remote_addr: SocketAddr) -> ConnectionInfo {
        ConnectionInfo {
            session_id: session_id,
            remote_addr: RemoteAddr(remote_addr),
        }
    }

    pub fn get_session_id(&self) -> Uuid {
        self.session_id
    }

    pub fn get_remote_addr(&self) -> &SocketAddr {
        &self.remote_addr.0
    }
}

impl Default for RemoteAddr {
    fn default() -> Self {
        RemoteAddr(SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 0))
    }
}


