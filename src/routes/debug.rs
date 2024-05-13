/*
    These are some debug endpoints
 */

use actix_web::{ get,Responder, HttpResponse, web::Data};
use crate::structs::{user::UserNoPassword, app_state::AppState};

#[get("/test_connection")]
pub async fn test_connection() -> impl Responder {
    HttpResponse::Ok().json("Connection appears to be okay")
}

#[get("/all_users")]
async fn get_all_users(state: Data<AppState>) -> impl Responder {
    let query = "SELECT users.user_id, username, first_name, last_name, email_address, phone_number, birthdate FROM users JOIN user_profiles USING(user_id)";
    
    match sqlx::query_as::<_,UserNoPassword>(
        query
    )
    .fetch_all(&state.db)
    .await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err))
    }
}

#[get("/test_auth")]
async fn test_auth() -> impl Responder {
   HttpResponse::Ok().json("Seems to work")
}