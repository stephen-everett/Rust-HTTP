use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};
use actix::Addr;

use crate::websockets::actors::waiting_room::WaitingRoom;

// Structure for connecting to postgres database
// Author: Stephen Everett
pub struct AppState {
    pub db: Pool<Postgres>,
    pub ws_server: Option<Addr <WaitingRoom>>
}

// Structure that contains data embedded in JWT token used for authentication
// More data can be embedded as necessary
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub user_id:String,
}