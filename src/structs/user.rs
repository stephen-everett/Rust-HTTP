use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize,FromRow,Deserialize)]
pub struct User{
    username: String,
    first_name:String,
    last_name:String,
    email_address:String,
    phone_number:String,
    birthdate:String,
    password:String,
    
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub username: String,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub phone_number:String,
    pub birthdate:String,
    pub password:String,
    pub pin:String
}

#[derive(Serialize, FromRow)]
pub struct UserNoPassword {
    pub user_id:String,
    pub username:String,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub phone_number:String,
    pub birthdate:String
}

#[derive(Serialize,FromRow)]
pub struct AuthUser {
    pub user_id:String,
    pub username:String,
    pub password: String,
}

#[derive(Serialize,FromRow)]
pub struct UserSearch{
    user_id: String,
    username:String,
    first_name:String,
    last_name:String
}



impl From<CreateUserBody> for User {
    fn from(user: CreateUserBody) -> Self {
        User{
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email_address:user.email_address,
            phone_number: user.phone_number,
            birthdate: user.birthdate,
            password: user.password,
        }
    }
}