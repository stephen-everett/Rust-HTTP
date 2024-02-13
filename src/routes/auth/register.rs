use serde::Serialize;
use sqlx::FromRow;
use actix_web::{post, web::{Data, Json}, Responder, HttpResponse};
use uuid::Uuid;
use argonautica::Hasher;


use crate::structs::{app_state::AppState, user::{CreateUserBody, UserNoPassword}};


#[derive(FromRow, Serialize)]
pub struct CountStruct {
    count:i64
}

#[post("/register")]
async fn create_user(state: Data<AppState>, body:Json<CreateUserBody>) -> impl Responder {

    // TODO: The explicit checks at the beginning of the function might be too much work.
    // When executing the main SQL queries near the bottom of the function, you should be 
    // able to narrow down what caused the error by matching the error in the match arms. The 
    // sqlx error is an enum
    // https://www.lpalmieri.com/posts/error-handling-rust/

    let user: CreateUserBody = body.into_inner();

    // bools to keep track of potential errors
    //let unique_email: bool = unique_email(state.clone(), user.email_address.clone()).await;
    //let unique_phone: bool = unique_phone(state.clone(), user.phone_number.clone()).await;
    //let unique_username = unique_username(state.clone(), user.username.clone()).await;
    let mut error = false;
    let  general_error = false;

    // container for uniqueness constraint errors
    let mut error_list: Vec<String> = Vec::new();


    // Return errors if there were any with uniqueness constrain checks
    if !unique_email(state.clone(), user.email_address.clone()).await {
        error_list.push("email".to_string());
        error = true;
    }
    if !unique_phone(state.clone(), user.phone_number.clone()).await {
        error_list.push("phone".to_string());
        error = true;
    }
    if !unique_username(state.clone(), user.username.clone()).await {
        error_list.push("username".to_string());
        error = true;
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
        match sqlx::query(
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

        // insert into user_profiles table
        match sqlx::query_as::<_, UserNoPassword>(
            "INSERT INTO user_profiles(user_id, username, profile_first_name, profile_last_name) VALUES($1,$2,$3,$4)"
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

async fn unique_email(state: Data<AppState>, email:String) -> bool {
    match sqlx::query_as::<_,CountStruct>(
        "SELECT COUNT(*) FROM users WHERE LOWER(email_address) = $1"
    )
    .bind(email.to_lowercase())
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => {
            if count.count > 0 {
                return false
            }
            else {
                return true
            }
        },
        Err(_) => return false
    }
}

async fn unique_phone(state: Data<AppState>, phone_number:String) -> bool {
    match sqlx::query_as::<_,CountStruct>(
        "SELECT COUNT(*) FROM users WHERE LOWER(phone_number) = $1"
    )
    .bind(phone_number.to_lowercase())
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => {
            if count.count > 0 {
                return false;
            }
            else {
                return true;
            }
        },
        Err(_) => return false
    }
}

async fn unique_username(state: Data<AppState>, username:String) -> bool{
    match sqlx::query_as::<_,CountStruct>(
        "SELECT COUNT(*) FROM user_profiles WHERE LOWER(username) = $1"
    )
    .bind(username.to_lowercase())
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => {
            if count.count > 0 {
                return false
            }
            else {
                return true
            }
        },
        Err(_err) => return false
    }
}