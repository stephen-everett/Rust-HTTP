use actix::{Actor, StreamHandler, AsyncContext};
use actix_web_actors::ws;
use serde::{Serialize,Deserialize};
use serde_json::{json, Value};
use actix::Message as ActixMessage;


#[derive(Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub code: MessageType,
    pub data: Value
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
    Info
}

pub struct ConnectedUser;

impl Actor for ConnectedUser {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ConnectedUser {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<UserMessage>(&text) {
                    Ok(content) => match content.code {
                        MessageType::Echo => {
                            ctx.text(content.data.to_string())
                        },
                        _ => ctx.text("uwu can't use that yet 👉👈"),
                    }
                    Err(err) => ctx.text(format!("uwu👉👈 there was an error {:?}", err))
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
