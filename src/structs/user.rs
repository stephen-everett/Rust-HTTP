/*
    Different user structures used throughout the program. When Serializing / Deserializing data
    To send over HTTP, there needs to be a matching structure used / defined.

    When receiving data from database, there needs to be a matching structure used/defined.
 */


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

/// User object but without password field. Used to 
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

// Structure used to retrieve user login request from front-end,
// and to retrieve data from DB
/*
    Author: Luis Baca
*/
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


/// Constructor to create a CreateUserBody from a User
/*
    Author: Luis Baca
 */
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