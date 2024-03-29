use actix::prelude::*;
use serde::{Deserialize, Serialize};
use crate::websockets::actors::{connected_user::ConnectedUser, lobby::Lobby};
use crate::structs::{receipt_item::ReceiptItem, lobby::LobbyReceipt};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Authorized {
    pub user_id: String,
    pub username: String,
    pub addr: Addr<Lobby>
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct AuthorizedUser {
    pub user_id: String,
    pub username: String,
    pub addr: Addr<ConnectedUser>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinedLobby {
   pub lobby_id:String
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct ServerMessage {
    pub context: String,
    pub code: String,
    pub data:MessageData
}

#[derive(Debug, Message, Serialize)]
#[rtype(result = "()")]
pub struct LobbyState {
    pub users: Vec<User>,
    pub receipt: LobbyReceipt
}

impl LobbyState {
    pub fn new(users: Vec<User>, receipt: LobbyReceipt) -> LobbyState {
        LobbyState {
            users:users,
            receipt: receipt
        }
    }
}

#[derive(Serialize, Debug)]
pub struct User {
    pub user_id:String,
    pub username: String,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct DUser {
    pub user_id:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemClaim {
    pub item_id: String,
    pub user_id: String
}


#[derive(Serialize)]
pub enum MessageData {
    ServerState(LobbyState),
    UserData(User),
    DisconnectedUser(DUser),
    Message(String),
    Claim(ItemClaim)
}
