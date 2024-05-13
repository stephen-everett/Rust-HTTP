/*
    These are generic messages used for some API calls
 */
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Message {
    id: i32,
    message:String,
}

#[derive(Deserialize)]
pub struct NewMessage {
    pub test: String,
}