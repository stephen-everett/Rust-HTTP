use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Lobby {
    pub lobby_id: i32
}