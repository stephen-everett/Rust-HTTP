use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use serde_json::{json, Value};
use crate::experimental::chat::actors::connected_user::{SocketMessage, UserMessage, MessageType};

/// Chat server sends this messages to session
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Debug)]
pub struct WaitingRoom {
    sessions: HashMap<String, Recipient<Message>>,
    rooms: HashMap<String, HashSet<String>>,
}

impl Actor for WaitingRoom {
    type Context = Context<Self>;
}

impl WaitingRoom {
    pub fn new() -> WaitingRoom {
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        WaitingRoom {
            sessions: HashMap::new(),
            rooms,
        }
    }
}

impl Handler<SocketMessage> for WaitingRoom {
    type Result = ();
    

    fn handle(&mut self, msg: SocketMessage, _: &mut Context<Self>)  {
      let response = SocketMessage {
        code: msg.code,
        data: Value::String("Hello from server".to_string()),
        addr: msg.addr
      };
      let message = Message("Hello".to_string());
      response.addr.send(message);
      //MessageResult(response)
    }
}

