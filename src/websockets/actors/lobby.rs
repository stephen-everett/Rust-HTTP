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
            User, ItemClaim, NewLobbyState, StartState, CheckoutItems
        },
        user_message::{Disconnect, MessageType, RemoveItem, SocketMessage},
    },
};
use actix::prelude::*;

use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::structs::{app_state::AppState, receipt_item::ReceiptItem};

use crate::websockets::queries::{get_receipt, claim_item, get_claims, unclaim_item, check_lobby, delete_lobby};
use crate::routes::app::post_receipt::create_lobby_room;


// definitions
pub struct Lobby {
    //sessions: HashMap<String, Recipient<Message>>,
    sessions: HashMap<String, actix::Addr<ConnectedUser>>,
    rooms: HashMap<String, ActiveLobby>,
    state: actix_web::web::Data<AppState>,
}

#[derive(Serialize, Clone)]
pub struct LobbyItemModifier {
    pub modifier_name: String,
    pub modifier_price: i64
}
#[derive(Serialize, Clone)]
pub struct ItemSplit {
    pub split: Vec<i64>,
    pub split_index: usize,
    pub price_balance: i64,
}
#[derive(Serialize, Clone)]
pub struct LobbyItem {
    pub receipt_item: ReceiptItem,
    pub modifiers: Vec<LobbyItemModifier>,
    pub claims: Vec<String>,
    pub in_checkout: Vec<String>,
    pub total_price: i64,
}
#[derive(Serialize, Clone)]
pub struct Item {
    pub item: LobbyItem,
    pub split: ItemSplit
}

#[derive(Clone)]
pub struct ActiveLobby {
    pub users: HashMap<String,LobbyUser>,
    pub items: HashMap<String, Item>
}

#[derive(Debug, Clone)]
pub struct LobbyUser {
    username: String,
    user_id: String,
    addr: actix::Addr<ConnectedUser>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateLobby {
    pub lobby_id:String,
    pub items: HashMap<String, Item>
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
                match msg.data.clone() {
                    Value::String(lobby_id) => {
                        match self.rooms.get_mut(&lobby_id) {
                            Some(lobby) => {
                                let mut users: Vec<User> = Vec::new();

                                let mut alreadyJoined: bool = false;
                                for (_, user) in lobby.users.iter() {
                                    if (user.user_id == msg.user_id) {
                                        alreadyJoined = true;
                                    }
                                }
                                if !alreadyJoined {
                                    for (_, user) in lobby.users.iter() {
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
    
                                    lobby.users.insert(msg.user_id.clone(), new_user);
                                    //msg.addr.do_send(Message(format!("Users in room: {:?}", name)));
                                    msg.addr.do_send(JoinedLobby {
                                        lobby_id: lobby_id.clone(),
                                    });
    
                                    users.push(User {
                                        username: msg.username.clone(),
                                        user_id: msg.user_id.clone(),
                                    });
    
                                    let somestate = self.state.clone();
                                    let users_cloned: Vec<User> = users.clone();
                                    let lobby_id_moved = lobby_id.clone();
                                    let msg_addr_cloned = msg.addr.clone();
                                    let future = async move {
                                        let receipt = get_receipt(somestate.clone(), lobby_id_moved.clone()).await;
                                        let claims = get_claims(somestate, lobby_id_moved).await;
                                        match receipt {
                                            Some(lobby_receipt) => {
                                                let lobby_state = LobbyState::new(users_cloned, lobby_receipt, claims.unwrap());
                                                let message = ServerMessage {
                                                    context: String::from("lobby"),
                                                    code: String::from("state"),
                                                    data: MessageData::ServerState(lobby_state),
                                                };
                                                msg_addr_cloned.do_send(message);
                                            }
                                            None => msg_addr_cloned.do_send(ServerMessage {
                                                context: String::from("lobby"),
                                                code: String::from("err"),
                                                data: MessageData::Message(
                                                    "Problem retrieving lobby state".to_string(),
                                                ),
                                            }),
                                        }
                                    };
                                    let mut temp_items:Vec<LobbyItem> = Vec::new();
                                    for (_, item) in &lobby.items {
                                        temp_items.push(item.item.clone());
                                    }
                                    msg.addr.do_send(ServerMessage{
                                        context: String::from("lobby"),
                                        code: String::from("state2"),
                                        data: MessageData::NewServerState(NewLobbyState { users: users, items: temp_items})
                                    });
                                    future.into_actor(self).spawn(ctx);
                                }
                                else {
                                    msg.addr.do_send(ServerMessage{
                                        context: String::from("lobby"),
                                        code: String::from("already joined"),
                                        data: MessageData::Message(String::from("User has already joined the lobby"))
                                    })
                                }
                            }
                            None => {
                                //msg.addr.do_send(Message(format!("Users in room: {:?}", name)));
                                
                                let cloned_state = self.state.clone();
                                let future = async move {
                                    let lobby_exist = check_lobby(cloned_state.clone(), lobby_id.clone()).await;
                                    match lobby_exist {
                                        true => {
                                            create_lobby_room(cloned_state, lobby_id.clone()).await;
                                            msg.addr.do_send(ServerMessage {
                                                context: String::from("lobby"),
                                                code: String::from("first"),
                                                data: MessageData::Message(String::from("First user to join"))
                                            });
                                        }
                                        false => {
                                            msg.addr.do_send(ServerMessage {
                                            context: String::from("lobby"),
                                            code: String::from("no lobby"),
                                            data: MessageData::Message(String::from("lobby does not exist"))
                                        });
                                    }
                                    }
                                    
    
                                };
                                future.into_actor(self).spawn(ctx);
                            }
                        }
                    }
                    _ => (),
                }
            }, // end of join
            MessageType::ItemClaim => { 
                // find lobby id and message data
                match self.rooms.get_mut(&msg.lobby_id) {
                    Some(lobby) => {
                        match msg.data {
                            Value::String(item_id) => {
                                // add claim to item in lobby
                                match lobby.items.get_mut(&item_id){
                                    Some(item) => {
                                        item.item.claims.push(msg.user_id.clone());
                                    },
                                    None => ()
                                }
                                // forward claim message to lobby
                                for (_, user) in lobby.users.iter() {
                                    user.addr.do_send(ServerMessage {
                                        context: String::from("lobby"),
                                        code: String::from("item_claim"),
                                        data: MessageData::Claim(ItemClaim {
                                            item_id: item_id.clone(),
                                            user_id: msg.user_id.clone(),
                                        }),
                                    })
                                }
                                // add claim to db
                                let somestate = self.state.clone();
                                let future = async move {
                                    let claim = claim_item(somestate, ItemClaim{
                                        item_id:item_id.clone(),
                                        user_id: msg.user_id.clone()
                                    }, msg.lobby_id.clone()).await;
                                    match claim {
                                        Ok(_) => (),
                                        Err(err) => msg.addr.do_send(ServerMessage{
                                            context: String::from("error"),
                                            code: String::from("database error"),
                                            data: MessageData::Message(err.to_string())
                                        })
                                    }
                                };
                                future.into_actor(self).spawn(ctx);
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                } 
            }, // end of item claim
            MessageType::ItemUnclaim => { 
                // find lobby id and message data
                match self.rooms.get_mut(&msg.lobby_id) {
                    Some(lobby) => {
                        match msg.data {
                            Value::String(item_id) => {
                                // remove claim to item in lobby
                                match lobby.items.get_mut(&item_id){
                                    Some(item) => {
                                        item.item.claims.retain(|x| *x != msg.user_id.clone());
                                    },
                                    None => ()
                                }
                                // forward claim message to lobby
                                for (_, user) in lobby.users.iter() {
                                    user.addr.do_send(ServerMessage {
                                        context: String::from("lobby"),
                                        code: String::from("item_unclaim"),
                                        data: MessageData::Claim(ItemClaim {
                                            item_id: item_id.clone(),
                                            user_id: msg.user_id.clone(),
                                        }),
                                    })
                                }
                                // add claim to db
                                let somestate = self.state.clone();
                                let future = async move {
                                    let claim = unclaim_item(somestate, ItemClaim{
                                        item_id:item_id.clone(),
                                        user_id: msg.user_id.clone()
                                    }).await;
                                    match claim {
                                        Ok(_) => (),
                                        Err(err) => msg.addr.do_send(ServerMessage{
                                            context: String::from("error"),
                                            code: String::from("database error"),
                                            data: MessageData::Message(err.to_string())
                                        })
                                    }
                                };
                                future.into_actor(self).spawn(ctx);
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                } 
            }, // end of item unclaim
            MessageType::StateRequest => {
                match msg.data {
                    Value::String(lobby_id) => {
                        match self.rooms.get_mut(&lobby_id) {
                            Some(lobby) => {
                                let mut users: Vec<User> = Vec::new();
                                for (_, user) in lobby.users.iter() {
                                    // collect names of everyone in room
                                    users.push(User {
                                        username: user.username.clone(),
                                        user_id: user.user_id.clone(),
                                    });
                                }

                                let somestate = self.state.clone();
                                let some_users = users.clone();
                                let lobby_id_moved = lobby_id.clone();
                                let msg_addr_clone = msg.addr.clone();
                                let future = async move {
                                    let receipt = get_receipt(somestate.clone(), lobby_id_moved.clone()).await;
                                    let claims = get_claims(somestate, lobby_id_moved).await;
                                    match receipt {
                                        Some(lobby_receipt) => {
                                            let lobby_state = LobbyState::new(some_users.clone(), lobby_receipt, claims.unwrap());
                                            let message = ServerMessage {
                                                context: String::from("lobby"),
                                                code: String::from("state"),
                                                data: MessageData::ServerState(lobby_state),
                                            };
                                            msg_addr_clone.do_send(message);
                                        }
                                        None => msg_addr_clone.do_send(ServerMessage {
                                            context: String::from("lobby"),
                                            code: String::from("err"),
                                            data: MessageData::Message(
                                                "Problem retrieving lobby state".to_string(),
                                            ),
                                        }),
                                    }
                                };
                                

                                let mut temp_items:Vec<LobbyItem> = Vec::new();
                                for (_, item) in &lobby.items {
                                    temp_items.push(item.item.clone());
                                }
                                msg.addr.do_send(ServerMessage{
                                    context: String::from("lobby"),
                                    code: String::from("state2"),
                                    data: MessageData::NewServerState(NewLobbyState { users: users, items: temp_items})
                                });
                                future.into_actor(self).spawn(ctx);
                            }
                            None => msg.addr.do_send(ServerMessage {
                                context: String::from("lobby"),
                                code: String::from("err"),
                                data: MessageData::Message(
                                    "Lobby does not exist".to_string(),
                                ),
                            }),
                        }
                    }
                    _ => (),
                }
            }, // end of state request
            MessageType::Checkout => {
                match self.rooms.get_mut(&msg.lobby_id) {
                    Some(lobby) => {
                        let mut claimed: Vec<String> = Vec::new();
                        let user_id = msg.user_id.clone();
                        for (_,item) in &mut lobby.items {
                            for claims in &item.item.claims {
                                if claims.to_string() == user_id {
                                    item.item.in_checkout.push(user_id.clone());
                                    claimed.push(item.item.receipt_item.receipt_item_id.clone())
                                }
                            }
                        }
                        for (_, user) in &lobby.users {
                            user.addr.do_send(ServerMessage{
                                context: String::from("lobby"),
                                code: String::from("checkout"),
                                data: MessageData::UserCheckout(CheckoutItems {
                                    user_id: msg.user_id.clone(),
                                    receipt_item_ids: claimed.clone()
                                })
                            })
                        }
                    },
                    None => ()
                }
            }, // end of checkout
            MessageType::LeaveCheckout => {
                match self.rooms.get_mut(&msg.lobby_id) {
                    Some(lobby) => {
                        let mut claimed: Vec<String> = Vec::new();
                        for (_,item) in &mut lobby.items {
                            item.item.in_checkout.retain(|x| x.to_string() != msg.user_id.clone());
                            for claims in &item.item.claims {
                                if claims.to_string() == msg.user_id.clone() {
                                    claimed.push(item.item.receipt_item.receipt_item_id.clone())
                                }
                            }
                        }
                        for (_, user) in &lobby.users {
                            user.addr.do_send(ServerMessage{
                                context: String::from("lobby"),
                                code: String::from("leave_checkout"),
                                data: MessageData::UserCheckout(CheckoutItems {
                                    user_id: msg.user_id.clone(),
                                    receipt_item_ids: claimed.clone()
                                })
                            })
                        }
                    },
                    None => ()
                }
            }, // end of leave checkout
            MessageType::Pay => {
                match self.rooms.get_mut(&msg.lobby_id) {
                    Some(lobby) => {
                        let mut all_claimed = true;
                        for (_,item) in &mut lobby.items {
                            if item.item.in_checkout.is_empty() {
                                all_claimed = false;
                            }
                        }

                        if !all_claimed {
                            msg.addr.do_send(ServerMessage {
                                context: String::from("lobby"),
                                code: String::from("items_left"),
                                data: MessageData::Message(String::from("There are still items not being checked out"))
                            });
                        }
                        else {
                            for (_, user) in &lobby.users {
                                user.addr.do_send(ServerMessage{
                                    context: String::from("lobby"),
                                    code: String::from("paying"),
                                    data: MessageData::Message(String::from(msg.user_id.clone())) 
                                })
                            }

                            // add payment check here
                            if lobby.users.len() == 1 {
                                self.rooms.remove((&msg.lobby_id));
                                msg.addr.do_send(ServerMessage {
                                    context: String::from("payment"),
                                    code: String::from("payment success"),
                                    data: MessageData::Message(String::from("Payment successful"))
                                });

                                let cloned_state = self.state.clone();
                                let future = async move {
                                    delete_lobby(cloned_state, msg.lobby_id.clone()).await;
                                };
                                future.into_actor(self).spawn(ctx);
                            }
                            else {
                                msg.addr.do_send(ServerMessage {
                                    context: String::from("payment"),
                                    code: String::from("payment success"),
                                    data: MessageData::Message(String::from("Payment successful"))
                                });
                                lobby.users.remove(&msg.user_id);
                            }
                        }
                    },
                    None => ()
                }
            }, // end of leave 
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

    fn handle(&mut self, msg: Disconnect, ctx: &mut Context<Self>) {
        self.sessions.remove(&msg.user_id);
        let somestate = self.state.clone();



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
                lobby.users.remove(&msg.user_id);
                for (_, user) in lobby.users.iter() {
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

        
        let future = async move {
            let _ = sqlx::query(
                "DELETE FROM item_claims WHERE user_id = $1"
            )
            .bind(msg.user_id.clone())
            .execute(&somestate.db).await;
        };
        future.into_actor(self).spawn(ctx);

    }
}

impl Handler<RemoveItem> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RemoveItem, _: &mut Context<Self>) {
        match self.rooms.get_mut(&msg.lobby_id) {
            Some(lobby) => {
                for (_, user) in lobby.users.iter() {
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

impl Handler<CreateLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: CreateLobby, _:&mut Context<Self>) {
        match self.rooms.get_mut(&msg.lobby_id.clone()) {
            Some(_) => (println!("lobby found!")),
            None => {
                println!("No lobby found");
                let temp_users: HashMap<String, LobbyUser> = HashMap::new();
                self.rooms.insert(
                    msg.lobby_id.clone(),
                    ActiveLobby{
                        users: temp_users,
                        items:msg.items.clone()
                    }

                );
            }
        }
    }
}

impl Handler<StartState> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: StartState, ctx: &mut Self::Context) {
        self.state = msg.state;     
    }
}