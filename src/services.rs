use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

#[derive(Serialize, FromRow)]
struct Message {
    id: i32,
    message:String,
}

#[get("/api/get/messages")]
pub async fn fetch_messages(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()

    match sqlx::query_as::<_, Message>("SELECT * FROM messages")
        .fetch_all(&state.db)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}
