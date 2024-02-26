use actix::{Message, Addr};

use crate::websockets::{
    actors::{waiting_room::WaitingRoom, lobby::Lobby},
    messages::user_message::{SocketMessage, Disconnect}
};

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
pub enum Server {
    WaitingRoom(Addr<WaitingRoom>),
    Lobby(Addr<Lobby>)
}

impl Server {
    pub fn do_send(&self, msg: SocketMessage) {
        match self {
            Server::WaitingRoom(addr) => addr.do_send(msg),
            Server::Lobby(addr) => addr.do_send(msg)
        }
    }
    pub fn send_disconnect(&self, msg: Disconnect) {
        match self {
            Server::WaitingRoom(addr) => addr.do_send(msg),
            Server::Lobby(addr) => addr.do_send(msg)
        }
    }
}