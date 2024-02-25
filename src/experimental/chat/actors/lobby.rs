use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use serde_json::{json, Value};
use crate::experimental::chat::actors::{
    connected_user::{SocketMessage, UserMessage, MessageType, Connect, ConnectedUser},
    waiting_room::Message
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct AuthorizedUser {
    pub addr: Addr<ConnectedUser>,
}

#[derive(Debug)]
pub struct Lobby {
    //sessions: HashMap<String, Recipient<Message>>,
    sessions: Vec<actix::Addr<ConnectedUser>>,
    rooms: HashMap<String, HashSet<String>>,
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Lobby {
    pub fn new() -> Lobby {
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());
        println!("Starting Lobby!");

        Lobby {
            sessions: Vec::new(),
            rooms,
        }
    }
}

impl Handler<AuthorizedUser> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: AuthorizedUser, _: &mut Context<Self>) {
        self.sessions.push(msg.addr);
        println!("User authorized! {:?}", self.sessions.len())
    }

}

impl Handler<SocketMessage> for Lobby {
    type Result = ();
    
    fn handle(&mut self, msg: SocketMessage, _: &mut Context<Self>) {
        match msg.code {
            MessageType:: Echo => {
                
            }
            _ => msg.addr.do_send(Message("Lobby hasn't implemented that yet!".to_string()))
        }
    }
}