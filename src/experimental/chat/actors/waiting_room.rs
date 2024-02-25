use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use serde_json::{json, Value};
use crate::experimental::chat::actors::{
    connected_user::{SocketMessage, UserMessage, MessageType, Connect, ConnectedUser, Server},
    lobby::{Lobby, AuthorizedUser}
};


#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageFromServer {
    pub code: MessageType,
    pub data: Value
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Authorized {
    pub addr: Addr<Lobby>
}


/// Chat server sends this messages to session
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);

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
                msg.addr.do_send(Authorized {
                    addr: self.lobby.clone()
                });
                self.lobby.do_send(AuthorizedUser {
                    addr:msg.addr
                });
            }
            _ => (msg.addr.do_send(Message("uwu Unauthorized 👉👈".to_string())))
            /* 
            MessageType::Info => {
                let message = MessageFromServer {
                    code: msg.code,
                    data: serde_json::Value::String("Hello this is a message from the server!".to_string())
                };
                msg.addr.do_send(message);
            },
            MessageType::Echo => {
                println!("Number of connected users: {:?}", self.sessions.len());
                for connection in self.sessions.iter() {
                    connection.do_send(Message(msg.data.to_string()));
                    connection.do_send(Message("Response from Echo in server".to_string()));
                }
            }
            _ => {
                let message = Message("Hello".to_string());
                msg.addr.do_send(message);
            }
            */
        }
    }
}

impl Handler<Connect> for WaitingRoom {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>)  {
        self.sessions.push(msg.addr);
        println!("Someoneone connected. Number of users in sessions: {:?}", self.sessions.len());
        println!("Address of the WaitingRoom server: {:?}", &ctx);
    }
}

