use actix_web::{
    get, post,
    web::{Data, Json},
    Responder, HttpResponse, web
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow,};
use crate::AppState;
use rand::Rng;

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

#[derive(Deserialize)]
pub struct MenuItem {
    pub SKU: i32,
    pub name: String,
    pub quantity: i32
}

#[derive(Serialize, FromRow)]
pub struct ReceiptItem {
    pub lobby_id: i32,
    pub SKU: i32,
    pub name: String,
    pub quantity: i32
}

#[derive(Serialize, Deserialize)]
pub struct Lobby {
    lobby_id: i32
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

#[post("/api/receipt/post_receipt")]
pub async fn post_receipt(state:Data<AppState>, body: web::Json<Vec<MenuItem>>) -> impl Responder {
    let lobby_number = futures::executor::block_on(create_new_lobby(state.clone()));
    for MenuItem in body.into_inner() {
        {
            match sqlx::query!("INSERT INTO menu_item(SKU, name, quantity) VALUES($1,$2,$3)",MenuItem.SKU, MenuItem.name, MenuItem.quantity)
                .execute(&state.db)
                .await 
                {
                    Ok(_) => (),
                    Err(_) => print!("Unable to insert menu item into table"),
                };

           
            match sqlx::query!("INSERT INTO receipt_item(lobby_id, SKU) VALUES($1,$2)", lobby_number, MenuItem.SKU)
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

#[post("/api/lobby/join")]
pub async fn join_lobby(state: Data<AppState>, body: Json<Lobby>) -> impl Responder {
    let query_string = format!("SELECT lobby_id, SKU, name, quantity FROM receipt_item JOIN lobby USING(lobby_id) JOIN menu_item USING(sku) WHERE lobby_id = {}", body.lobby_id);
    match sqlx::query_as!(
        ReceiptItem,
        "SELECT lobby_id, SKU AS \"SKU\", name, quantity FROM receipt_item JOIN lobby USING(lobby_id) JOIN menu_item USING(sku) WHERE lobby_id = $1",
        body.lobby_id
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json( query_string),
    }
    
}


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
