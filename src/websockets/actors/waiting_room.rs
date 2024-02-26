
use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::VerifyWithKey;
use sqlx::prelude::FromRow;


use crate::{
    websockets::{
        actors::{connected_user::ConnectedUser, lobby::Lobby},
        messages::{
            user_message::{SocketMessage, MessageType, Connect, Disconnect},
            server_message::{Authorized, AuthorizedUser, Message}
        },
        queries::get_username,
    },
    structs::app_state::{TokenClaims, AppState}
};

#[derive(FromRow)]
pub struct UserName {
    username: String
}

//#[derive(Debug)]
pub struct WaitingRoom {
    //sessions: HashMap<String, Recipient<Message>>,
    sessions: HashMap<String, actix::Addr<ConnectedUser>>,
    rooms: HashMap<String, HashSet<String>>,
    lobby: Addr<Lobby>,
    state: actix_web::web::Data<AppState>
}

impl Actor for WaitingRoom {
    type Context = Context<Self>;
}

impl WaitingRoom {
    pub fn new(state:actix_web::web::Data<AppState>) -> WaitingRoom {
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());
        println!("Starting WaitingRoom!");

        WaitingRoom {
            sessions: HashMap::new(),
            rooms,
            lobby: Lobby::new(state.clone()).start(),
            state:state.clone()
        }
    }
}

impl Handler<SocketMessage> for WaitingRoom {
    type Result = ();
    
    fn  handle(&mut self, msg: SocketMessage, ctx: &mut Context<Self>)  {
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

                        // Handle TokenClaims, or return error
                        match claims {
                            Ok(value) => {
                                // remove ws from sessions
                                self.sessions.remove(&msg.user_id);
                                println!("User authorized and moved out of lobby. Number of users in sessions: {:?}", self.sessions.len());
                                
                                let somestate = self.state.clone();
                                let lobby_addr = self.lobby.clone();

                                let future = async move {
                                    let username = get_username(somestate, value.user_id.clone()).await;
                                    msg.addr.do_send(Message(format!("Username pulled from DB: {:?},", username.to_string())));
                                    
                                    // send lobby address to ConenctedUser
                                    msg.addr.do_send(Authorized {
                                        username: username.clone(),
                                        user_id: value.user_id.clone(),
                                        addr: lobby_addr.clone()
                                    });

                                    // Send ConnectUser address to Lobby
                                    lobby_addr.do_send(AuthorizedUser {
                                        username: username,
                                        user_id: value.user_id,
                                        addr:msg.addr
                                    });
                                };
                                future.into_actor(self).spawn(ctx);

                                
                                
                            },
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
        self.sessions.insert(msg.user_id, msg.addr);
        println!("Someoneone connected. Number of users in sessions: {:?}", self.sessions.len());
    }
}

impl Handler<Disconnect> for WaitingRoom {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Context<Self>)  {
        self.sessions.remove(&msg.user_id);
        
        for (_, addr) in self.sessions.iter() {
            addr.do_send(Message(format!("User disconnected. Current users: {:?}", self.sessions.len())))
        }
    }
}