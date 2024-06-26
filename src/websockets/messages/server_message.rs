/*
    These are the messages that the server sends to the user
 */

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use crate::websockets::actors::{connected_user::ConnectedUser, lobby::{Lobby, LobbyItem}};
use crate::structs::lobby::LobbyReceipt;
use actix_web::web::Data;
use crate::structs::app_state::AppState; 



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
    pub receipt: LobbyReceipt,
    pub claims: Vec<ItemClaim>
}

impl LobbyState {
    pub fn new(users: Vec<User>, receipt: LobbyReceipt, claims:Vec<ItemClaim>) -> LobbyState {
        LobbyState {
            users:users,
            receipt: receipt,
            claims: claims
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct User {
    pub user_id:String,
    pub username: String,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct DUser {
    pub user_id:String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ItemClaim {
    pub item_id: String,
    pub user_id: String
}

#[derive(Serialize)]
pub struct NewLobbyState {
    pub users: Vec<User>,
    pub items: Vec<LobbyItem>
}

#[derive(Serialize)]
pub struct CheckoutItems {
    pub user_id: String,
    pub receipt_item_ids: Vec<String>
}

#[derive(Serialize)]
pub enum MessageData {
    ServerState(LobbyState),
    UserData(User),
    DisconnectedUser(DUser),
    Message(String),
    Claim(ItemClaim),
    NewServerState(NewLobbyState),
    UserCheckout(CheckoutItems)
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartState {
    pub state: Data<AppState>
}
