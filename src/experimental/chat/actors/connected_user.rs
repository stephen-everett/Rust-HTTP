use actix::{Actor, StreamHandler, AsyncContext};
use actix_web_actors::ws;
use serde::{Serialize,Deserialize};
use serde_json::{json, Value};
use actix::Message as ActixMessage;
use actix::prelude::*;


use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::VerifyWithKey;

use crate::experimental::chat::actors::{
    waiting_room::{WaitingRoom, Message, MessageFromServer, Authorized},
    lobby::Lobby
};
use crate::structs::app_state::TokenClaims;

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
pub enum Server {
    WaitingRoom(Addr<WaitingRoom>),
    Lobby(Addr<Lobby>)
}


#[derive(Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub code: MessageType,
    pub data: Value
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<ConnectedUser>,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct SocketMessage {
    pub code: MessageType,
    pub data: Value,
    pub addr: Addr<ConnectedUser>,
}


impl UserMessage {
    pub fn err(msg:String) -> Self {
        UserMessage {
            code: MessageType::Err,
            data: json!(msg)
        }
    }

    pub fn info(msg:String) -> Self {
        UserMessage {
            code: MessageType::Info,
            data: json!(msg)
        }
    }
}

impl Into<String> for UserMessage {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Echo,
    Join,
    Err,
    Info,
    Auth
}

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

impl Server {
    fn do_send(&self, msg: SocketMessage) {
        match self {
            Server::WaitingRoom(addr) => addr.do_send(msg),
            Server::Lobby(addr) => addr.do_send(msg)
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
                    /*match content.code {
                        MessageType::Echo => {
                            //ctx.text(content.data.to_string())
                            let addr = ctx.address();
                            self.addr.do_send( SocketMessage {
                                code: content.code,
                                data: content.data,
                                addr: addr
                            })
                        },
                        MessageType::Auth => {
                            let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT SECRET must be set!");
                            let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

                            match content.data.as_str() {
                                Some(str) => {
                                    // grab token from credentials passed from request
                                    let token_string = str;

                                    // check to see if token  is valid
                                    let claims: Result<TokenClaims, &str> = token_string
                                        .verify_with_key(&key)
                                        .map_err(|_| "Invalid Token");

                                    match claims {
                                        Ok(value) => ctx.text(value.user_id),
                                        Err(_) => ctx.text("Didn't authenticate")
                                    }
                                }    
                                _ => ctx.text("👉👈 wrong data")
                            }
                            //let token_string: &str = content.data.as_str();

                        },
                        MessageType::Join => {
                            let addr = ctx.address();
                            ctx.text("Hit the join area");
                            self.addr.send(Connect {
                                addr: ctx.address()
                            })
                            .into_actor(self)
                            .then(|res, _, ctx| {
                               match res {
                                Ok(response) => ctx.text(""),
                                _ => ctx.text("Something went wrong")
                               }
                               fut::ready(())
                            })
                            .wait(ctx)
                               
                        },
                        MessageType::Info => {
                            let addr = ctx.address();
                            ctx.text("Hit the info area");
                            self.addr.send(SocketMessage {
                                code: content.code,
                                data: content.data,
                                addr: ctx.address()
                            })
                            .into_actor(self)
                            .then(|res, _, ctx| {
                               match res {
                                Ok(response) => ctx.text(""),
                                _ => ctx.text("Something went wrong")
                               }
                               fut::ready(())
                            })
                            .wait(ctx)

                        }
                        _ => ctx.text("uwu can't use that yet 👉👈"),
                        */
                    
                    Err(err) => ctx.text(format!("uwu👉👈 there was an error {:?}", err))
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<UserMessage> for ConnectedUser {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, ctx: &mut  ws::WebsocketContext<Self>) -> Self::Result {
        match msg.code {
            MessageType::Info => {
                println!("Hello");
                ctx.text(msg.data.to_string())
            },
            _ => ctx.text("Got something from the server but not implemented yet")
        }
    }
}

impl Handler<Message> for ConnectedUser {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut  ws::WebsocketContext<Self>) {
       ctx.text(msg.0);
    }
}

impl Handler<MessageFromServer> for ConnectedUser {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer, ctx: &mut  ws::WebsocketContext<Self>) {
        println!("Handling a MessageFromServer");
        ctx.text(msg.data.to_string());
     }
}

impl Handler<Authorized> for ConnectedUser {
    type Result = ();

    fn handle(&mut self, msg: Authorized, ctx: &mut  ws::WebsocketContext<Self>) {
        self.addr = Server::Lobby(msg.addr);
    }
}
