use crate::structs::menu_item::MenuItem;
use crate::structs::app_state::AppState;
use actix_web::{  
    web::{Data, Json},
    Responder, HttpResponse,
};


async fn add_menu_item(state:Data<AppState>, body:Json<MenuItem>) -> impl Responder {
    match sqlx::query!("INSERT INTO menu_item(sku, name, quantity, price) VALUES($1,$2,$3,$4)", body.sku, body.name, body.quantity, body.price)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("successfully added menu item"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add menu item"),
    }
}