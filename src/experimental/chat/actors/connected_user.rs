use actix::{Actor, StreamHandler, AsyncContext};
use actix_web_actors::ws;
use serde::{Serialize,Deserialize};
use serde_json::{json, Value};
use actix::Message as ActixMessage;


use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::VerifyWithKey;

use crate::structs::app_state::TokenClaims;



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
    Info,
    Auth
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
                        MessageType::Auth => {
                            let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT SECRET must be set!");
                            let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

                            match content.data.as_str() {
                                Some(str) => {
                                    // grab token from credentials passed from request
                                    let token_string = str;

                                    // check to see if token  is valid
                                    let claims: Result<TokenClaims, &str> = token_string
                                        .verify_with_key(&key)
                                        .map_err(|_| "Invalid Token");

                                    match claims {
                                        Ok(value) => ctx.text("Successfully authenticated"),
                                        Err(_) => ctx.text("Didn't authenticate")
                                    }
                                }    
                                _ => ctx.text("👉👈 wrong data")
                            }
                            //let token_string: &str = content.data.as_str();

                        }
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
