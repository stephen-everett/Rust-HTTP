use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow,};



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



#[derive(Serialize,Deserialize,FromRow)]
struct BankInformation{
    bank_name: String,
    bank_routing: String,
    bank_account_number:String
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

#[derive(Serialize,FromRow)]
struct FriendList{
    friend_list: Vec<User>,
    pending_friend_request: Vec<User>,
    accept_friend: bool,
    decline_friend: bool,

}

/// structure for request when creating user
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

/// structure to return to client without password
#[derive(Serialize, FromRow)]
pub struct UserNoPassword {
    pub id:i32,
    pub user_name:String,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub phone_number:String,
    pub birthdate:String
}

// structure for SQL return for auth user
#[derive(Serialize,FromRow)]
pub struct AuthUser {
    pub user_id:String,
    pub username:String,
    pub password: String,
}


// The users bank information is going to be in this format
// #[derive(Serialize,Deserialize,FromRow)]
// struct BankInformation{
//     bank_name: String,
//     bank_routing: String,
//     bank_account_number:String
// }
