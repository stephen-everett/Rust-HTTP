use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Lobby {
    lobby_id: i32
}