/*
    Author: Stephen Everett

    This is the first server that the user connects to. The only message that it will handle is an Auth message.
    Any other messages are denied. Once Auth goes through correctly, it will send a message to the connected user
    actor with the address for the lobby. Inside lobby the user can then join rooms, or send information normally.
 */


// imports
use std::collections::HashMap;
use actix::prelude::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::VerifyWithKey;


use crate::{
    websockets::{
        actors::{connected_user::ConnectedUser, lobby::Lobby},
        messages::{
            user_message::{SocketMessage, MessageType, Connect, Disconnect, RemoveItem},
            server_message::{Authorized, AuthorizedUser, ServerMessage, MessageData}
        },
        queries::get_username,
    },
    structs::app_state::{TokenClaims, AppState}
};

/*
    Definitions
 */

//#[derive(Debug)]
pub struct WaitingRoom {
    sessions: HashMap<String, actix::Addr<ConnectedUser>>,
    lobby: Addr<Lobby>,
    state: actix_web::web::Data<AppState>
}

impl Actor for WaitingRoom {
    type Context = Context<Self>;
}

impl WaitingRoom {
    pub fn new(state:actix_web::web::Data<AppState>) -> WaitingRoom {
        println!("Starting WaitingRoom!");

        WaitingRoom {
            sessions: HashMap::new(),
            lobby: Lobby::new(state.clone()).start(),
            state:state.clone()
        }
    }
}

/*
    Handler for messages from client. Will only reply to Auth messages, all others are deined
 */
impl Handler<SocketMessage> for WaitingRoom {
    type Result = ();
    
    fn  handle(&mut self, msg: SocketMessage, ctx: &mut Context<Self>)  {
        match msg.code {
            // Handle auth message. If authorized, reply with address to Lobby
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
                                    msg.addr.do_send(ServerMessage{
                                        context: String::from("waiting_room"),
                                        code: String::from("debug"),
                                        data: MessageData::Message(format!("Username pulled from DB: {:?},", username.to_string())),
                                    });
                                    
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
                            Err(_) => msg.addr.do_send(ServerMessage {
                                context: String::from("error"),
                                code: String::from("auth"),
                                data: MessageData::Message(String::from("Didn't authenticate"))
                            })
                        }
                    }    
                    _ => msg.addr.do_send(ServerMessage {
                            context: String::from("error"),
                            code: String::from("auth"),
                            data: MessageData::Message(String::from("👉👈 wrong data"))
                        })
                }
            }
            _ => msg.addr.do_send(ServerMessage {
                    context: String::from("error"),
                    code: String::from("auth"),
                    data: MessageData::Message(String::from("uwu Unauthorized 👉👈"))
                })
        }
    }
}

// handle connect message from connected_user
impl Handler<Connect> for WaitingRoom {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>)  {
        self.sessions.insert(msg.user_id, msg.addr);
        println!("Someoneone connected. Number of users in sessions: {:?}", self.sessions.len());
    }
}

// handle disconnect message from connected_user
impl Handler<Disconnect> for WaitingRoom {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>)  {
        self.sessions.remove(&msg.user_id);
        
        /* 
        for (_, addr) in self.sessions.iter() {
            addr.do_send(Message(format!("User disconnected. Current users: {:?}", self.sessions.len())))
        }
        */
    }
}

impl Handler<RemoveItem> for WaitingRoom {
    type Result = ();

    fn handle(&mut self, msg:RemoveItem, ctx: &mut Context<Self>) {
        self.lobby.do_send(msg)
    }
}