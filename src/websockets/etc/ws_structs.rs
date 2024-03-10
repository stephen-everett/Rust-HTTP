use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct WsUser {
    pub user_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name:String,
    pub email_address:String,
    pub birthdate: String,
    pub phone_number: String,
}