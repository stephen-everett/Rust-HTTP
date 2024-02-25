
use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::VerifyWithKey;

use crate::{
    websockets::{
        actors::{connected_user::ConnectedUser, lobby::Lobby},
        messages::{
            user_message::{SocketMessage, MessageType, Connect},
            server_message::{Authorized, AuthorizedUser, Message}
        },
    },
    structs::app_state::TokenClaims
};


#[derive(Debug)]
pub struct WaitingRoom {
    //sessions: HashMap<String, Recipient<Message>>,
    sessions: Vec<actix::Addr<ConnectedUser>>,
    rooms: HashMap<String, HashSet<String>>,
    lobby: Addr<Lobby>
}

impl Actor for WaitingRoom {
    type Context = Context<Self>;
}

impl WaitingRoom {
    pub fn new() -> WaitingRoom {
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());
        println!("Starting WaitingRoom!");

        WaitingRoom {
            sessions: Vec::new(),
            rooms,
            lobby: Lobby::new().start()
        }
    }
}

impl Handler<SocketMessage> for WaitingRoom {
    type Result = ();
    
    fn handle(&mut self, msg: SocketMessage, _: &mut Context<Self>)  {
        match msg.code {
            MessageType::Auth => {
                let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT SECRET must be set!");
                let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

                match msg.data.as_str() {
                    Some(str) => {
                        // grab token from credentials passed from request
                        let token_string = str;

                        // check to see if token  is valid
                        let claims: Result<TokenClaims, &str> = token_string
                            .verify_with_key(&key)
                            .map_err(|_| "Invalid Token");

                        match claims {
                            Ok(value) => msg.addr.do_send(Message("Authentication seems to work".to_string())),
                            Err(_) => msg.addr.do_send(Message("Didn't authenticate".to_string()))
                        }
                    }    
                    _ => msg.addr.do_send(Message("👉👈 wrong data".to_string()))
                }
                //let token_string: &str = content.data.as_str();
                /* 
                msg.addr.do_send(Authorized {
                    addr: self.lobby.clone()
                });
                self.lobby.do_send(AuthorizedUser {
                    addr:msg.addr
                });
                */
            }
            _ => msg.addr.do_send(Message("uwu Unauthorized 👉👈".to_string()))
        }
    }
}

impl Handler<Connect> for WaitingRoom {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>)  {
        self.sessions.push(msg.addr);
        println!("Someoneone connected. Number of users in sessions: {:?}", self.sessions.len());
    }
}