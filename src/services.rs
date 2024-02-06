use actix_web::{
    get, post,
    web::{Data, Json, ReqData},
    Responder, HttpResponse, web
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{digest::typenum::Integer, Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use serde::{Deserialize, Serialize};
use sqlx::{self, error::DatabaseError, postgres::PgDatabaseError, Database, FromRow};
use crate::{AppState, TokenClaims};
use rand::Rng;
use std::io;

use uuid::Uuid;

// structure for messages retrieved from DB
#[derive(Serialize, FromRow)]
struct Message {
    id: i32,
    message:String,
}
///
#[derive(Serialize,FromRow)]
struct UserSearch{
    username:String,
    first_name:String,
    last_name:String
}

// changed from searchPram to SearchParam to fix spelling and to fix warning from Cargo
#[derive(Deserialize)]
pub struct SearchParam{
    message:String
}

// structure for request when creating user
#[derive(Deserialize)]
struct CreateUserBody {
    username: String,
    first_name:String,
    last_name:String,
    email_address:String,
    phone_number:String,
    birthdate:String,
    password:String,
    pin:i32
}

// structure to return to client without password
#[derive(Serialize, FromRow)]
struct UserNoPassword {
    id:String,
    username:String,
    first_name:String,
    last_name:String,
    email_address:String,
    phone_number:String,
    birthdate:String
}

// structure for SQL return for auth user
#[derive(Serialize,FromRow)]
struct AuthUser {
    id:i32,
    user_name:String,
    password: String,
}


// structure for messages received from client
#[derive(Deserialize)]
pub struct NewMessage {
    pub test: String,
}

#[derive(Deserialize)]
pub struct MenuItem {
    pub sku: i32,
    pub name: String,
    pub quantity: i32
}

#[derive(Serialize, FromRow)]
pub struct ReceiptItem {
    pub lobby_id: i32,
    pub sku: i32,
    pub name: String,
    pub quantity: i32
}

#[derive(Serialize, Deserialize)]
pub struct Lobby {
    lobby_id: i32
}

#[derive(FromRow)]
pub struct CountStruct {
    count:i64
}

#[post("/api/register")]
async fn create_user(state: Data<AppState>, body:Json<CreateUserBody>) -> impl Responder {

    // TODO: The implicit checks at the beginning of the function might be too much work.
    // When executing the main SQL queries near the bottom of the function, you should be 
    // able to narrow down what caused the error by matching the error in the match arms
    // https://www.lpalmieri.com/posts/error-handling-rust/
    
    let user: CreateUserBody = body.into_inner();

    // bools to keep track of potential errors
    let mut duplicate_email = false;
    let mut duplicate_phone = false;
    let mut duplicate_username = false;
    let mut error = false;
    let mut general_error = false;

    // container for uniqueness constraint errors
    let mut error_list: Vec<String> = Vec::new();


    
    /**************************************************
        Uniqueness Constrain Checks
     **************************************************/

     // Check email
    match sqlx::query_as::<_,CountStruct>(
        "SELECT COUNT(*) FROM users WHERE LOWER(email_address) = $1"
    )
    .bind(user.email_address.to_lowercase())
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => {
            if count.count > 0 {
                duplicate_email = true;
                error = true;
            }
        },
        Err(err) => return HttpResponse::InternalServerError().json(format!("Something went wrong checking email:\n {:?}", err))
    }

    // Check phone number
    match sqlx::query_as::<_,CountStruct>(
        "SELECT COUNT(*) FROM users WHERE LOWER(phone_number) = $1"
    )
    .bind(user.phone_number.to_lowercase())
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => {
            if count.count > 0 {
                duplicate_phone = true;
                error = true;
            }
        },
        Err(err) => return HttpResponse::InternalServerError().json(format!("Something went wrong checking phone number:\n {:?}", err))
    }

    // Check username
    match sqlx::query_as::<_,CountStruct>(
        "SELECT COUNT(*) FROM user_profile WHERE LOWER(username) = $1"
    )
    .bind(user.username.to_lowercase())
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => {
            if count.count > 0 {
                duplicate_username = true;
                error = true;
            }
        },
        Err(err) => return HttpResponse::InternalServerError().json(format!("Something went wrong checking username:\n {:?}", err))
    }

    /*******************************************************************************************************************************************
     ******************************************************************************************************************************************/

    // Return errors if there were any with uniqueness constrain checks
    if duplicate_email {
        error_list.push("email".to_string());
    }
    if duplicate_phone {
        error_list.push("phone".to_string());
    }
    if duplicate_username {
        error_list.push("username".to_string());
    }
    if error {
        return HttpResponse::Conflict().json(error_list)
    }
    // Add new user to DB
    else {
        // Generate user ID and hash the password
        let id = Uuid::new_v4().to_string();
        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET needs to be set!");
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(user.password)
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();
        
        // insert user into user table
        match sqlx::query_as::<_, UserNoPassword>(
            "INSERT INTO USERS(user_id, first_name, last_name, email_address, phone_number, birthdate, password, pin)
            VALUES($1,$2,$3,$4,$5,$6,$7,$8)"
        )
        .bind(id.clone())
        .bind(user.first_name.clone())
        .bind(user.last_name.clone())
        .bind(user.email_address)
        .bind(user.phone_number)
        .bind(user.birthdate) 
        .bind(hash)
        .bind(user.pin)
        .fetch_one(&state.db)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                error_list.push(err.to_string());
            }
        }

        // insert into user_profile table
        match sqlx::query_as::<_, UserNoPassword>(
            "INSERT INTO user_profile(user_id, username, profile_first_name, profile_last_name) VALUES($1,$2,$3,$4)"
        )
        .bind(id)
        .bind(user.username)
        .bind(user.first_name)
        .bind(user.last_name)
        .fetch_one(&state.db)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                error_list.push(err.to_string());
            }
        }

        // return responses
        if general_error {
            return HttpResponse::InternalServerError().json(format!("Something went wrong:\n ${:?}", error_list))
        }
        else {
            return HttpResponse::Ok().json(format!("User added"))
        }
    }
}

///
#[post("/api/search")]
pub async fn search_user(state:Data<AppState>,body:Json<SearchParam>) -> impl Responder{
    
    // I think this is expecting user input from the console.
    /* 
    let mut target = String::new(); 
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read");
    */

    // get search parameter from body
    let search_param: SearchParam = body.into_inner();

    // query
    match sqlx::query_as::<_,UserSearch>(
    "SELECT user_name, first_name, last_name FROM users WHERE user_name = $1 OR first_name = $1 "
    )
    .bind(search_param.message)
    .fetch_all(&state.db)
    .await
    {
        //Ok(UserSearch) => HttpResponse::Ok().json(UserSearch),
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("User not found") 

    }
}

#[get("/api/auth")]
async fn basic_auth(state:Data<AppState>, credentials:BasicAuth) -> impl Responder {
    let jwt_secret:Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET").expect("JWT_SECRET needs to be set!").as_bytes()).unwrap();

    let user_name = credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password!"),
        Some(pass) => {
            match sqlx::query_as::<_,AuthUser>(
                "SELECT id, user_name, password FROM users WHERE user_name = $1",
            )
            .bind(user_name.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(user) => {
                    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(user.password)
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();
                    
                    if is_valid {
                        let claims = TokenClaims {id: user.id};
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    }
                    else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
        }
    }
    
}


// return all messages
#[get("/api/get/messages")]
pub async fn fetch_messages(state: Data<AppState>) -> impl Responder {

    match sqlx::query_as::<_, Message>("SELECT * FROM messages")
        .fetch_all(&state.db)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json("No messages found"),
    }
}

// insert a new message into the message table
#[post("/api/post/message")]
pub async fn post_message(state: Data<AppState>, body: Json<NewMessage>) -> impl Responder {
    match sqlx::query!("INSERT INTO messages(message) VALUES($1)", &body.test)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("successfully posted new message"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to post message"),
    }
}

// test HTTP connection. Returns Okay if connection was successful 
#[get("/api/test_connection")]
pub async fn test_connection() -> impl Responder {
    HttpResponse::Ok().json("Connection appears to be okay")
}

// delete all of the messages in the messages table
#[get("/api/messages/clear")]
pub async fn clear_messages(state:Data<AppState>) -> impl Responder {
    match sqlx::query!("DELETE FROM messages")
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Messages have been cleared"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to clear messages"),
    }
}

// retrieve lobby number, insert menu items into their respective table, and then create the realationship between
// menu item and lobby by inserting items into receipt_item table
#[post("/api/receipt/post_receipt")]
pub async fn post_receipt(state:Data<AppState>, body: web::Json<Vec<MenuItem>>) -> impl Responder {
    let lobby_number = futures::executor::block_on(create_new_lobby(state.clone()));
    for menu_item in body.into_inner() {
        {
            match sqlx::query!("INSERT INTO menu_item(SKU, name, quantity) VALUES($1,$2,$3)",menu_item.sku, menu_item.name, menu_item.quantity)
                .execute(&state.db)
                .await 
                {
                    Ok(_) => (),
                    Err(_) => print!("Unable to insert menu item into table"),
                };

           
            match sqlx::query!("INSERT INTO receipt_item(lobby_id, SKU) VALUES($1,$2)", lobby_number, menu_item.sku)
                .execute(&state.db)
                .await 
                {
                    Ok(_) => (),
                    Err(_) => print!("Unable to insert receipt item into table"),
                };
        }
    }
    HttpResponse::Ok().json(lobby_number)
}

// return all menu items associated with a given lobby number
#[post("/api/lobby/join")]
pub async fn join_lobby(state: Data<AppState>, body: Json<Lobby>) -> impl Responder {
    let query_string = format!("SELECT lobby_id, SKU, name, quantity FROM receipt_item JOIN lobby USING(lobby_id) JOIN menu_item USING(sku) WHERE lobby_id = {}", body.lobby_id);
    match sqlx::query_as!(
        ReceiptItem,
        "SELECT lobby_id, sku, name, quantity FROM receipt_item JOIN lobby USING(lobby_id) JOIN menu_item USING(sku) WHERE lobby_id = $1",
        body.lobby_id
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json( query_string),
    }
    
}

// create a new lobby and return lobby number
pub async fn create_new_lobby(state:Data<AppState>) -> i32 {
    let mut rng = rand::thread_rng();
    let lobby_number = rng.gen::<i32>();
    match sqlx::query!("INSERT INTO lobby VALUES($1)",lobby_number)
    .execute(&state.db)
    .await
    {
        Ok(_) => lobby_number,
        Err(_) => 0
    };
    lobby_number
}

