
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
use sqlx::{self, error::DatabaseError, postgres::PgDatabaseError, Database, FromRow, Row};
use uuid::Uuid;
use crate::{AppState, TokenClaims};
use rand::Rng;

use crate::services::profile::{CreateUserBody,UserNoPassword,AuthUser};


#[derive(FromRow, Serialize)]
pub struct CountStruct {
    count:i64
}



#[post("/api/register")]
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
    let mut general_error = false;

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
        Err(err) => return false
    }
}




#[get("/api/auth")]
async fn basic_auth(state:Data<AppState>, credentials:BasicAuth) -> impl Responder {
    let jwt_secret:Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET").expect("JWT_SECRET needs to be set!").as_bytes()).unwrap();

    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password!"),
        Some(pass) => {
            match sqlx::query_as::<_,AuthUser>(
                "SELECT users.user_id, username, password FROM users JOIN user_profiles USING (user_id) WHERE username = $1",
            )
            .bind(username.to_string())
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
                        let claims = TokenClaims {user_id: user.user_id};
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    }
                    else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                Err(err) =>{
                    match err {
                        sqlx::Error::RowNotFound => {
                            HttpResponse::Unauthorized().json("Incorrect username or password")
                        },
                        _ => HttpResponse::InternalServerError().json(format!("{:?}", err)),
                    }
                }
                //Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
        }
    }
    
}




#[get("/api/delete_user")]
async fn delete_user(state: Data<AppState>, claims: Option<web::ReqData<TokenClaims>>) -> impl Responder {
    
    match claims {
        Some(claims) => {
            let query = "DELETE FROM users WHERE user_id = $1";
            match sqlx::query(query)
                .bind(claims.user_id.to_string())
                .execute(&state.db)
                .await {
                    Ok(rows) => HttpResponse::Ok().json(format!("User has been deleted: {:?}", rows.rows_affected())),
                    Err(err) => HttpResponse::InternalServerError().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }

}


#[get("/api/test_auth")]
async fn test_auth() -> impl Responder {
   HttpResponse::Ok().json("Seems to work")
}