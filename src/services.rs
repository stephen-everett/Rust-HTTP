use actix_web::{
    get, post,
    web::{Data, Json},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow,};
use crate::AppState;

// structure for messages retrieved from DB
#[derive(Serialize, FromRow)]
struct Message {
    id: i32,
    message:String,
}

// structure for messages received from client
#[derive(Deserialize)]
pub struct NewMessage {
    pub test: String,
}

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

#[get("/api/test_connection")]
pub async fn test_connection() -> impl Responder {
    HttpResponse::Ok().json("Connection appears to be okay")
}

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
