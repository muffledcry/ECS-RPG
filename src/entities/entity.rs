use uuid::Uuid;
use crate::utils::ConnectionInfo;

#[derive(Clone, Debug)]
pub struct Entity {
    pub entity_id: Uuid,
    pub connection_info: ConnectionInfo,
}

impl Entity {
    pub fn new(entity_id: Uuid, connection_info: ConnectionInfo) -> Self {
        Self {
            entity_id,
            connection_info,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.entity_id
    }

    pub fn get_session_id(&self) -> Uuid {
        self.connection_info.session_id
    }

    pub fn get_connection_info(&self) -> &ConnectionInfo {
        &self.connection_info
    }
}
