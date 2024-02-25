use actix::prelude::*;


use crate::websockets::actors::{connected_user::ConnectedUser, lobby::Lobby};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Authorized {
    pub addr: Addr<Lobby>
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct AuthorizedUser {
    pub addr: Addr<ConnectedUser>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);