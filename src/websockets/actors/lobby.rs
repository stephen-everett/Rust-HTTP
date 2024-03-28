/*
   Author: Stephen Everett

   This is the main room that handles most of the connected_user messages.
   This is where all current connections are managed, as well as a collection of all
   lobbies that users can connect to.

   Handles messages and then updates the users as necessary
*/

// imports
use crate::websockets::{
    actors::connected_user::ConnectedUser,
    messages::{
        server_message::{
            AuthorizedUser, DUser, JoinedLobby, LobbyState, Message, MessageData, ServerMessage,
            User, ItemClaim
        },
        user_message::{Disconnect, MessageType, RemoveItem, SocketMessage},
    },
};
use actix::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

use crate::structs::app_state::AppState;

use crate::websockets::queries::get_receipt;

// definitions
pub struct Lobby {
    //sessions: HashMap<String, Recipient<Message>>,
    sessions: HashMap<String, actix::Addr<ConnectedUser>>,
    rooms: HashMap<String, HashMap<String, LobbyUser>>,
    state: actix_web::web::Data<AppState>,
}

#[derive(Debug)]
pub struct LobbyUser {
    username: String,
    user_id: String,
    addr: actix::Addr<ConnectedUser>,
}

// constructor
impl LobbyUser {
    pub fn new(username: String, user_id: String, addr: actix::Addr<ConnectedUser>) -> LobbyUser {
        LobbyUser {
            username: username,
            user_id: user_id,
            addr: addr,
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Lobby {
    pub fn new(state: actix_web::web::Data<AppState>) -> Lobby {
        println!("Starting Lobby!");

        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            state: state,
        }
    }
}

// receive an authorized user and add them to collection
impl Handler<AuthorizedUser> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: AuthorizedUser, _: &mut Context<Self>) {
        for (_, address) in self.sessions.iter() {
            address.do_send(ServerMessage {
                context: String::from("server"),
                code: String::from("debug"),
                data: MessageData::Message(format!("{:?} Joined!", msg.username)),
            })
        }
        self.sessions.insert(msg.user_id, msg.addr);
        println!("User authorized! {:?}", self.sessions.len())
    }
}

// handle messages from connected_user
impl Handler<SocketMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: SocketMessage, ctx: &mut Context<Self>) {
        match msg.code {
            MessageType::Echo => {
                for (_, address) in self
                    .sessions
                    .iter()
                    .filter(|(user_id, _)| **user_id != msg.user_id)
                {
                    address.do_send(Message(msg.data.to_string()))
                }
            }
            // Join a lobby given a lobby ID. If there is no lobby created, create one first. Reply back
            // to everyone in the lobby that someone has joined
            MessageType::Join => {
                match msg.data {
                    Value::String(lobby_id) => {
                        match self.rooms.get_mut(&lobby_id) {
                            Some(lobby) => {
                                let mut users: Vec<User> = Vec::new();
                                for (_, user) in lobby.iter() {
                                    //user.addr.do_send(Message(format!("{:?} has joined lobby", msg.username)));
                                    user.addr.do_send(ServerMessage {
                                        context: String::from("lobby"),
                                        code: String::from("user_join"),
                                        data: MessageData::UserData(User {
                                            user_id: String::from(&msg.user_id),
                                            username: String::from(&msg.username),
                                        }),
                                    });
                                    // collect names of everyone in room
                                    users.push(User {
                                        username: user.username.clone(),
                                        user_id: user.user_id.clone(),
                                    });
                                }
                                let new_user = LobbyUser::new(
                                    msg.username.clone(),
                                    msg.user_id.clone(),
                                    msg.addr.clone(),
                                );

                                lobby.insert(msg.user_id.clone(), new_user);
                                //msg.addr.do_send(Message(format!("Users in room: {:?}", name)));
                                msg.addr.do_send(JoinedLobby {
                                    lobby_id: lobby_id.clone(),
                                });

                                users.push(User {
                                    username: msg.username.clone(),
                                    user_id: msg.user_id.clone(),
                                });

                                let somestate = self.state.clone();
                                let users = users;
                                let lobby_id_moved = lobby_id.clone();
                                let future = async move {
                                    let receipt = get_receipt(somestate, lobby_id_moved).await;
                                    match receipt {
                                        Some(lobby_receipt) => {
                                            let lobby_state = LobbyState::new(users, lobby_receipt);
                                            let message = ServerMessage {
                                                context: String::from("lobby"),
                                                code: String::from("state"),
                                                data: MessageData::ServerState(lobby_state),
                                            };
                                            msg.addr.do_send(message);
                                        }
                                        None => msg.addr.do_send(ServerMessage {
                                            context: String::from("lobby"),
                                            code: String::from("err"),
                                            data: MessageData::Message(
                                                "Problem retrieving lobby state".to_string(),
                                            ),
                                        }),
                                    }
                                };
                                future.into_actor(self).spawn(ctx);
                            }
                            None => {
                                let new_user = LobbyUser::new(
                                    msg.username.clone(),
                                    msg.user_id.clone(),
                                    msg.addr.clone(),
                                );
                                self.rooms.insert(
                                    lobby_id.clone(),
                                    HashMap::from([(msg.user_id.clone(), new_user)]),
                                );
                                msg.addr.do_send(JoinedLobby {
                                    lobby_id: lobby_id.clone(),
                                });

                                let somestate = self.state.clone();
                                let users = Vec::from([User {
                                    username: msg.username.clone(),
                                    user_id: msg.user_id.clone(),
                                }]);
                                let lobby_id_moved = lobby_id.clone();
                                let future = async move {
                                    let receipt = get_receipt(somestate, lobby_id_moved).await;
                                    match receipt {
                                        Some(lobby_receipt) => {
                                            let lobby_state = LobbyState::new(users, lobby_receipt);
                                            let message = ServerMessage {
                                                context: String::from("lobby"),
                                                code: String::from("state"),
                                                data: MessageData::ServerState(lobby_state),
                                            };
                                            msg.addr.do_send(message);
                                        }
                                        None => msg.addr.do_send(ServerMessage {
                                            context: String::from("lobby"),
                                            code: String::from("err"),
                                            data: MessageData::Message(
                                                "Problem retrieving lobby state".to_string(),
                                            ),
                                        }),
                                    }
                                };
                                future.into_actor(self).spawn(ctx);
                            }
                        }
                    }
                    _ => (),
                }
            }
            MessageType::ItemClaim => { 
                match self.rooms.get_mut(&msg.lobby_id) {
                    Some(lobby) => {
                        match msg.data {
                            Value::String(item_id) => {
                                for (_, user) in lobby.iter() {
                                    user.addr.do_send(ServerMessage {
                                        context: String::from("lobby"),
                                        code: String::from("item_claim"),
                                        data: MessageData::Claim(ItemClaim {
                                            item_id: item_id.clone(),
                                            user_id: msg.user_id.clone(),
                                        }),
                                    })
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                } 
            }, // end of item claim
            _ => msg.addr.do_send(ServerMessage {
                context: String::from("error"),
                code: String::from("not found"),
                data: MessageData::Message(String::from("Lobby hasn't implemented that yet!")),
            }),
        }
    }
}

// send disconnect update to users when someone disconnects
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.user_id);

        for (_, user) in self.sessions.iter() {
            user.do_send(ServerMessage {
                context: String::from("server"),
                code: String::from("debug"),
                data: MessageData::Message(String::from(format!(
                    "Someone has disconnected from the server. Connections remaining: {:?}",
                    self.sessions.len()
                ))),
            })
        }

        match self.rooms.get_mut(&msg.lobby_id) {
            Some(lobby) => {
                lobby.remove(&msg.user_id);
                for (_, user) in lobby.iter() {
                    user.addr.do_send(ServerMessage {
                        context: String::from("lobby"),
                        code: String::from("user_leave"),
                        data: MessageData::DisconnectedUser(DUser {
                            user_id: msg.user_id.clone(),
                        }),
                    })
                }
            }
            None => (),
        }
    }
}

impl Handler<RemoveItem> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RemoveItem, _: &mut Context<Self>) {
        match self.rooms.get_mut(&msg.lobby_id) {
            Some(lobby) => {
                for (_, user) in lobby.iter() {
                    user.addr.do_send(ServerMessage {
                        context: String::from("lobby"),
                        code: String::from("remove_item"),
                        data: MessageData::Message(String::from(msg.item_id.clone())),
                    })
                }
            }
            None => (),
        }
    }
}
