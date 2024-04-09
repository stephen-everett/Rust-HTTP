/*
   Author: Stephen Everett
   Endpoints used for plugin to send over a receipt. It then creates a lobby, and enters the data
   into the database. Returns the Lobby ID to be used to join the lobby. Has a join_lobby endpoint for
   testing purposes, but actual joining is handled with the websockets
*/

use crate::structs::{
    app_state::AppState,
    lobby::{
        ItemModifier, Lobby, LobbyReceipt, ResturauntMenuItem, ResturauntReceipt, StateHeader,
        UpdateItem,
    },
    receipt_item::ReceiptItem,
};

use crate::websockets::{actors::waiting_room::WaitingRoom, messages::user_message::RemoveItem};
use actix_web::{
    post, web,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_actors::ws;
use sqlx::Error;
use uuid::Uuid;

// retrieve lobby number, insert menu items into their respective table, and then create the realationship between
// menu item and lobby by inserting items into receipt_item table
#[post("/post_receipt")]
pub async fn post_receipt(
    state: Data<AppState>,
    body: web::Json<ResturauntReceipt>,
) -> impl Responder {
    println!("Inserting lobby...");
    match create_new_lobby(state.clone()).await {
        Ok(lobby_id) => {
            println!("Lobby inserted: {:?}", lobby_id.clone());
            println!("Creating receipt...");
            match create_receipt(state.clone(), body.res_id.clone(), lobby_id.clone()).await {
                Ok(receipt_id) => {
                    println!("Receipt created: {:?}", receipt_id.clone());
                    match insert_menu_items(
                        state.clone(),
                        body.res_id.clone(),
                        receipt_id.clone(),
                        body.menu_items.clone(),
                    )
                    .await
                    {
                        Some(error) => HttpResponse::InternalServerError()
                            .json(format!("Problem inserting menu items {:?}", error)),
                        None => HttpResponse::Ok().json(lobby_id),
                    }
                }
                Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Problem creating lobby {:?}", err))
        }
    }
}
// create a new lobby and return lobby number
pub async fn create_new_lobby(state: Data<AppState>) -> Result<String, Error> {
    let lobby_number = Uuid::new_v4().to_string();
    match sqlx::query("INSERT INTO lobbies VALUES($1)")
        .bind(lobby_number.clone())
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok(lobby_number),
        Err(err) => Err(err),
    }
}

pub async fn create_receipt(
    state: Data<AppState>,
    res_id: String,
    lobby_id: String,
) -> Result<String, Error> {
    let receipt_id = Uuid::new_v4().to_string();
    println!("Receipt id created... {:?}", receipt_id.clone());
    match sqlx::query("INSERT INTO receipts(receipt_id, res_id, lobby_id) VALUES($1,$2,$3)")
        .bind(receipt_id.clone())
        .bind(res_id)
        .bind(lobby_id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            println!("Inserted receipt {:?} ", receipt_id.clone());
            Ok(receipt_id)
        }
        Err(err) => Err(err),
    }
}

pub async fn insert_menu_items(
    state: Data<AppState>,
    res_id: String,
    receipt_id: String,
    menu_items: Vec<ResturauntMenuItem>,
) -> Option<Error> {
    for item in menu_items {
        let item_id = Uuid::new_v4().to_string();
        match sqlx::query(
            "INSERT INTO menu_items(item_id, res_id, sku, name, price) VALUES($1,$2,$3,$4,$5)",
        )
        .bind(item_id.clone())
        .bind(res_id.clone())
        .bind(item.sku)
        .bind(item.name)
        .bind(item.price)
        .execute(&state.db)
        .await
        {
            Ok(_) => {
                let receipt_item_id = Uuid::new_v4().to_string();
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
            Err(err) => return Some(err),
        };
    }
    return None;
}

// return all menu items associated with a given lobby number
#[post("/get_receipt")]
pub async fn get_receipt(state: Data<AppState>, body: Json<Lobby>) -> impl Responder {
    match get_receipt_items(state.clone(), body.lobby_id.clone()).await {
        Some(receipt_items) => match resolve_header(state.clone(), body.lobby_id.clone()).await {
            Some(header) => match get_mods(state, body.lobby_id.clone()).await {
                Some(mods) => {
                    let state = LobbyReceipt {
                        header: header,
                        menu_items: receipt_items,
                        modifiers: mods,
                    };
                    HttpResponse::Ok().json(state)
                }
                None => HttpResponse::InternalServerError().json("Could not locate modifiers"),
            },
            None => HttpResponse::InternalServerError().json("Could not locate header"),
        },
        None => HttpResponse::InternalServerError().json("Could not retrieve menu items"),
    }
}

pub async fn resolve_header(state: Data<AppState>, lobby_id: String) -> Option<StateHeader> {
    match sqlx::query_as::<_, StateHeader>(
        "SELECT res_id, lobby_id, receipt_id FROM receipts WHERE lobby_id = $1",
    )
    .bind(lobby_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(header) => Some(header),
        Err(_) => None,
    }
}

pub async fn get_receipt_items(
    state: Data<AppState>,
    lobby_id: String,
) -> Option<Vec<ReceiptItem>> {
    match sqlx::query_as::<_, ReceiptItem>(
        "
        SELECT receipt_item_id,sku, name, price FROM (
            SELECT receipt_id, receipt_item_id, item_id
            FROM receipts JOIN receipt_items USING(receipt_id)
            WHERE lobby_id = $1
            )
        AS receipt JOIN menu_items USING(item_id)",
    )
    .bind(lobby_id)
    .fetch_all(&state.db)
    .await
    {
        Ok(receipt_items) => Some(receipt_items),
        Err(_) => None,
    }
}

pub async fn get_mods(state: Data<AppState>, lobby_id: String) -> Option<Vec<ItemModifier>> {
    match sqlx::query_as::<_, ItemModifier>(
        "
        SELECT receipt_item_id, modifier_name AS name, modifier_price AS price FROM (
            SELECT receipt_item_id
            FROM receipts
             JOIN receipt_items USING (receipt_id)
            WHERE lobby_id = $1
            ) AS receiptItems JOIN item_modifiers USING(receipt_item_id)",
    )
    .bind(lobby_id)
    .fetch_all(&state.db)
    .await
    {
        Ok(mods) => Some(mods),
        Err(err) => None,
    }
}

#[post("/delete_item")]
pub async fn delete_item(state: Data<AppState>, item: Json<UpdateItem>) -> impl Responder {
    match &state.ws_server {
        Some(server) => {
            server.do_send(RemoveItem {
                item_id: item.item_id.clone(),
                lobby_id:item.lobby_id.clone()
            })
        }
        None => ()
    };
    match sqlx::query(
        "DELETE FROM menu_items WHERE item_id = (
            SELECT item_id FROM receipt_items WHERE receipt_item_id = $1
            )",
    )
    .bind(item.item_id.clone())
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().json("Item deleted"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
