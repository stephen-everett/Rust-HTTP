use serde::{Serialize,Deserialize};
use serde_json::Value;
use actix::Message as ActixMessage;
use actix::prelude::*;

use crate::websockets::actors::connected_user::ConnectedUser;


#[derive(Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub code: MessageType,
    pub data: Value
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SocketMessage {
    pub code: MessageType,
    pub data: Value,
    pub addr: Addr<ConnectedUser>,
    pub user_id:String,
    pub username:String
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Echo,
    Join,
    Err,
    Info,
    Auth
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: String,
    pub addr: Addr<ConnectedUser>,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: String,
    pub lobby_id:String
}