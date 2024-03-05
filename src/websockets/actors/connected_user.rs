use actix::{Actor, StreamHandler, AsyncContext};
use actix_web_actors::ws;
use actix::prelude::*;
use rand::Rng;
use std::time::{Duration, Instant};

use crate::websockets::{
    etc::connection_pool::Server,
    messages::{
        user_message::{UserMessage, SocketMessage, Connect, Disconnect},
        server_message::{Message, Authorized, JoinedLobby, ServerMessage}
    },
};

use crate::websockets::actors::lobby::LobbyState;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct ConnectedUser{
    pub user_id: String,
    pub username: String,
    pub room: String,
    pub addr: Server,
    pub hb: Instant,
    pub lobby_id: String,
}
impl ConnectedUser {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");

                act.addr.send_disconnect(Disconnect {
                    lobby_id: act.lobby_id.clone(),
                    user_id: act.user_id.clone()
                });

                ctx.stop();

                return;


            }

            ctx.ping(b"");
        });
    }
}

impl Actor for ConnectedUser {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context){
        self.hb(ctx);
        match &self.addr {
            Server::WaitingRoom(addr) => {
                let mut rng = rand::thread_rng();
                let lobby_number = rng.gen::<i32>();
                self.user_id = lobby_number.to_string();

                addr.do_send(Connect {
                    user_id: lobby_number.to_string(),
                    addr: ctx.address()
                }
                )},
            _ => ()
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ConnectedUser {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
                self.hb = Instant::now();
            },
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            },
            Ok(ws::Message::Close(_)) => {
                self.addr.send_disconnect(Disconnect {
                    user_id: self.user_id.clone(),
                    lobby_id: self.lobby_id.clone()
                });
                ctx.stop();
            }
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<UserMessage>(&text) {
                    Ok(content) => self.addr.do_send(SocketMessage {
                        username: self.username.clone(),
                        code: content.code,
                        data: content.data,
                        addr: ctx.address(),
                        user_id: self.user_id.clone()
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
        self.username = msg.username;
        self.user_id = msg.user_id;
        self.addr = Server::Lobby(msg.addr);
    }
}

impl Handler<JoinedLobby> for ConnectedUser {
    type Result = ();

    fn handle(&mut self, msg: JoinedLobby, _ctx: &mut  ws::WebsocketContext<Self>) {
        self.lobby_id = msg.lobby_id;
    }
}

impl Handler<LobbyState> for ConnectedUser {
    type Result = ();
    
    fn handle(&mut self, msg: LobbyState, ctx: &mut  ws::WebsocketContext<Self>) {
        ctx.text(serde_json::to_string(&msg).unwrap());

    }
}

impl Handler<ServerMessage> for ConnectedUser {
    type Result = ();
    
    fn handle(&mut self, msg: ServerMessage, ctx: &mut  ws::WebsocketContext<Self>) {
        ctx.text(serde_json::to_string(&msg).unwrap());

    }
}