use crate::structs::menu_item::MenuItem;
use crate::structs::app_state::AppState;
use actix_web::{  
    web::{Data, Json},
    Responder, HttpResponse,
};


async fn update_menu_item(state: Data<AppState>, body: Json<MenuItem>) -> impl Responder {
    let up_current = "UPDATE menu_item SET name = $1, quantity = $2, price = $3 WHERE sku = $4";
    match sqlx::query(up_current)
        .bind(&body.name)
        .bind(&body.quantity)
        .bind(&body.price)
        .bind(&body.sku)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("successfully updated menu item"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}