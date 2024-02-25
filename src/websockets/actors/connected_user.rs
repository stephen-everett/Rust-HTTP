use actix::{Actor, StreamHandler, AsyncContext};
use actix_web_actors::ws;
use actix::prelude::*;

use crate::websockets::{
    etc::connection_pool::Server,
    messages::{
        user_message::{UserMessage, SocketMessage, Connect},
        server_message::{Message, Authorized}
    },
};


#[derive(Debug)]
pub struct ConnectedUser{
    pub user_id: String,
    pub username: String,
    pub room: String,
    pub addr: Server
}
impl ConnectedUser {}

impl Actor for ConnectedUser {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context){
        match &self.addr {
            Server::WaitingRoom(addr) => addr.do_send(Connect {
                addr: ctx.address()
            }),
            _ => ()
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ConnectedUser {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<UserMessage>(&text) {
                    Ok(content) => self.addr.do_send(SocketMessage {
                        code: content.code,
                        data: content.data,
                        addr: ctx.address()
                    }),
                    Err(err) => ctx.text(format!("uwu👉👈 there was an error {:?}", err))
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<Message> for ConnectedUser {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut  ws::WebsocketContext<Self>) {
       ctx.text(msg.0);
    }
}

impl Handler<Authorized> for ConnectedUser {
    type Result = ();

    fn handle(&mut self, msg: Authorized, _ctx: &mut  ws::WebsocketContext<Self>) {
        self.addr = Server::Lobby(msg.addr);
    }
}