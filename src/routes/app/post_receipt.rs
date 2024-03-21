/*
    Author: Stephen Everett
    Endpoints used for plugin to send over a receipt. It then creates a lobby, and enters the data
    into the database. Returns the Lobby ID to be used to join the lobby. Has a join_lobby endpoint for 
    testing purposes, but actual joining is handled with the websockets
 */

use actix::fut::err;
use actix_web::{
    post,
    web::{Data, Json},
    Responder, HttpResponse, web
};
use uuid::Uuid;
use sqlx::Error;
use crate::structs::{
    app_state::AppState,
    menu_item::MenuItem,
    receipt_item::ReceiptItem,
    lobby::{Lobby, ResturauntMenuItem, ResturauntReceipt }
};
use rand::Rng;

// retrieve lobby number, insert menu items into their respective table, and then create the realationship between
// menu item and lobby by inserting items into receipt_item table
#[post("/post_receipt")]
pub async fn post_receipt(state:Data<AppState>, body: web::Json<ResturauntReceipt>) -> impl Responder {
    println!("Inserting lobby...");
    match futures::executor::block_on(create_new_lobby(state.clone())){
        Ok(lobby_id) => {
            println!("Lobby inserted: {:?}", lobby_id.clone());
            println!("Creating receipt...");
            match create_receipt(state.clone(), body.res_id.clone(),lobby_id.clone()).await{
                Ok(receipt_id) => {
                    println!("Receipt created: {:?}", receipt_id.clone());
                    match insert_menu_items(state.clone(), body.res_id.clone(),receipt_id.clone(),body.menu_items.clone()).await {
                        Some(error) => HttpResponse::InternalServerError().json(format!("Problem inserting menu items {:?}", error)),
                        None => HttpResponse::Ok().json(lobby_id)
                    }
                },
                Err(err) => HttpResponse::InternalServerError().json(err.to_string())
            }
            
        },
        Err(err) => HttpResponse::InternalServerError().json(format!("Problem creating lobby {:?}", err))
    }
}
// create a new lobby and return lobby number
pub async fn create_new_lobby(state:Data<AppState>) -> Result<String, Error> {
    let lobby_number = Uuid::new_v4().to_string();
    match sqlx::query("INSERT INTO lobbies VALUES($1)")
    .bind(lobby_number.clone())
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(lobby_number),
        Err(err) => Err(err)
    }
}

pub async fn create_receipt(state:Data<AppState>, res_id:String, lobby_id:String) -> Result<String, Error> {
    let receipt_id = Uuid::new_v4().to_string();
    println!("Receipt id created... {:?}", receipt_id.clone());
    match sqlx::query(
        "INSERT INTO receipts(receipt_id, res_id, lobby_id) VALUES($1,$2,$3)"
    )
    .bind(receipt_id.clone())
    .bind(res_id)
    .bind(lobby_id)
    .execute(&state.db)
    .await
    {
        Ok(_) => {
            println!("Inserted receipt {:?} ", receipt_id.clone());
            Ok(receipt_id)
        },
        Err(err) => Err(err)
    }
}

pub async fn insert_menu_items(state:Data<AppState>, res_id:String, receipt_id:String, menu_items:Vec<ResturauntMenuItem>) -> Option<Error> {
    for item in menu_items{
        let item_id = Uuid::new_v4().to_string();
        match sqlx::query(
            "INSERT INTO menu_items(item_id, res_id, sku, name, price) VALUES($1,$2,$3,$4,$5)"
        )
        .bind(item_id.clone())
        .bind(res_id.clone())
        .bind(item.sku)
        .bind(item.name)
        .bind(item.price)
        .execute(&state.db)
        .await{
            Ok(_) => {
                let receipt_item_id =Uuid::new_v4().to_string();
                match sqlx::query(
                    "INSERT INTO receipt_items(receipt_item_id, receipt_id, item_id) VALUES($1,$2,$3)"
                )
                .bind(receipt_item_id.clone())
                .bind(receipt_id.clone())
                .bind(item_id.clone())
                .execute(&state.db)
                .await
                {
                    Ok(_) => {
                        for modifier in item.modifiers {
                                match sqlx::query(
                                    "INSERT INTO item_modifiers(modifier_name, modifier_price, receipt_item_id) VALUES($1,$2,$3)"
                                )
                                .bind(modifier.name)
                                .bind(modifier.price)
                                .bind(receipt_item_id.clone())
                                .execute(&state.db)
                                .await {
                                    Ok(_) => (),
                                    Err(err) => return Some(err)
                                }
                        }
                    }
                    Err(err) => return Some(err)
                }
            }
            Err(err) => return Some(err)
        };
    }
    return None
}
// return all menu items associated with a given lobby number
/* 
#[post("/join_lobby")]
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
*/