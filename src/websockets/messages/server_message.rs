use actix::prelude::*;
use serde::Serialize;
use std::fmt;
use crate::websockets::actors::{connected_user::ConnectedUser, lobby::Lobby};
use crate::structs::receipt_item::ReceiptItem;

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
    pub users: Vec<String>,
    pub menu_items: Vec<ReceiptItem>
}

impl LobbyState {
    pub fn new(users: Vec<String>, menu_items: Vec<ReceiptItem>) -> LobbyState {
        LobbyState {
            users:users,
            menu_items:menu_items
        }
    }
}


pub enum MessageData {
    ServerState(LobbyState)
}
