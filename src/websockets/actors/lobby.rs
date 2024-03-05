use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use futures::future::Join;
use serde::Serialize;
use serde_json::{Value, json};
use crate::websockets::{
    actors::connected_user::ConnectedUser,
    messages::{
        user_message::{SocketMessage, MessageType, Disconnect},
        server_message::{AuthorizedUser, Message, JoinedLobby, ServerMessage}
    },
};

use crate::structs::{
    receipt_item::ReceiptItem,
    app_state::AppState
};

use crate::websockets::queries::get_receipt;

pub struct Lobby {
    //sessions: HashMap<String, Recipient<Message>>,
    sessions: HashMap<String, actix::Addr<ConnectedUser>>,
    rooms: HashMap<String, HashMap<String,LobbyUser>>,
    state:actix_web::web::Data<AppState>
}

#[derive(Debug)]
pub struct LobbyUser {
    username:String,
    user_id: String,
    addr: actix::Addr<ConnectedUser>
}

impl LobbyUser {
    pub fn new(username: String, user_id:String, addr:actix::Addr<ConnectedUser>) -> LobbyUser {
        LobbyUser {
            username:username,
            user_id:user_id,
            addr:addr
        }
    }
}

#[derive(Debug, Message, Serialize)]
#[rtype(result = "()")]
pub struct LobbyState {
    users: Vec<String>,
    menu_items: Vec<ReceiptItem>
}

impl LobbyState {
    pub fn new(users: Vec<String>, menu_items: Vec<ReceiptItem>) -> LobbyState {
        LobbyState {
            users:users,
            menu_items:menu_items
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Lobby {
    pub fn new(state:actix_web::web::Data<AppState>) -> Lobby {
        println!("Starting Lobby!");

        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            state:state
        }
    }
}

impl Handler<AuthorizedUser> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: AuthorizedUser, _: &mut Context<Self>) {
        for (_, address) in self.sessions.iter() {
            address.do_send(Message(format!("{:?} Joined!", msg.username)))
        }
        self.sessions.insert(msg.user_id,msg.addr);
        println!("User authorized! {:?}", self.sessions.len())
    }

}

impl Handler<SocketMessage> for Lobby {
    type Result = ();
    
    fn handle(&mut self, msg: SocketMessage, ctx: &mut Context<Self>) {
        match msg.code {
            MessageType:: Echo => {
                for (_,address) in self.sessions.iter().filter(|(user_id, _)| **user_id != msg.user_id) {
                    address.do_send(Message(msg.data.to_string()))
                }
            }
            MessageType::Join => {
                match msg.data{
                    Value::String(lobby_id) => {
                        match self.rooms.get_mut(&lobby_id){
                            Some(lobby) => {
                                let mut names: Vec<String> = Vec::new();
                                for (_, user) in lobby.iter() {
                                    //user.addr.do_send(Message(format!("{:?} has joined lobby", msg.username)));
                                    user.addr.do_send(ServerMessage {
                                        context: String::from("lobby"),
                                        code: String::from("user_join"),
                                        data: String::from(&msg.username)
                                    });
                                    names.push(user.username.clone()); // collect names of everyone in room
                                }
                                let new_user  = LobbyUser::new(msg.username.clone(), msg.user_id.clone(), msg.addr.clone());
                    
                                lobby.insert(msg.user_id, new_user);
                                //msg.addr.do_send(Message(format!("Users in room: {:?}", name)));
                                msg.addr.do_send(JoinedLobby {
                                    lobby_id:lobby_id.clone()
                                });

                                names.push(msg.username.clone());

                                let somestate = self.state.clone();
                                let users = names;
                                let lobby_id_moved = lobby_id.clone();
                                let future = async move {
                                    let receipt = get_receipt(somestate, lobby_id_moved).await;
                                    let lobby_state = LobbyState::new(users, receipt);
                                    let message = ServerMessage {
                                        context: String::from("lobby"),
                                        code: String::from("state"),
                                        data: serde_json::to_string(&lobby_state).unwrap(),
                                    };
                                    msg.addr.do_send(message);
                                };
                                future.into_actor(self).spawn(ctx);


                            },
                            None => {
                                let new_user  = LobbyUser::new(msg.username.clone(), msg.user_id.clone(), msg.addr.clone());
                                self.rooms.insert(lobby_id.clone(), HashMap::from([(msg.user_id, new_user)]));
                                msg.addr.do_send(JoinedLobby {
                                    lobby_id:lobby_id.clone()
                                });

                                let somestate = self.state.clone();
                                let users = Vec::from([msg.username.clone()]);
                                let lobby_id_moved = lobby_id.clone();
                                let future = async move {
                                    let receipt = get_receipt(somestate, lobby_id_moved).await;
                                    let lobby_state = LobbyState::new(users, receipt);
                                    msg.addr.do_send(lobby_state);
                                };
                                future.into_actor(self).spawn(ctx);

                            }
                        }
                    },
                    _ => ()
                }
                
            }
            _ => msg.addr.do_send(Message("Lobby hasn't implemented that yet!".to_string()))
        }
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.user_id);

        for (_,user) in self.sessions.iter() {
            user.do_send(Message(format!("Someone has disocnnected from the session: {:?}", self.sessions.len())))
        }

        match self.rooms.get_mut(&msg.lobby_id){
            Some(lobby) => {
                lobby.remove(&msg.user_id);
                for (_, user) in lobby.iter() {
                    user.addr.do_send(Message(format!("Someone has disconnected from the lobby! Users remaining: {:?}", lobby.len())))
                }
            },
            None => (),
        }
    }

}