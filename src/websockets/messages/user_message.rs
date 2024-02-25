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
    pub addr: Addr<ConnectedUser>,
}