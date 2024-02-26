use actix::prelude::*;


use crate::websockets::actors::{connected_user::ConnectedUser, lobby::Lobby};

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