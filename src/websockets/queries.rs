/*
    These are queries used for websockets
 */


use actix_web::web::Data;
use sqlx::FromRow;
use crate::{
    structs::{
        app_state::AppState, lobby::LobbyReceipt
    },
    routes::app::post_receipt::{get_receipt_items, resolve_header, get_mods},
    websockets::messages::server_message::ItemClaim
};

#[derive(FromRow)]
pub struct UserName {
    username: String
}

pub async fn get_username(state:Data<AppState>, id:String) -> String{
    // get search parameter from body
    // query
    match sqlx::query_as::<_,UserName>(
        "SELECT username FROM user_profiles WHERE user_id = $1"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(data) => data.username,
        Err(_) => "UsernameError".to_string()
    }
}

pub async fn get_receipt(state:Data<AppState>, id:String) -> Option<LobbyReceipt>{
   match get_receipt_items(state.clone(), id.clone()).await {
        Some(receipt_items) => {
            match resolve_header(state.clone(), id.clone()).await {
                Some(header) => {
                    match get_mods(state.clone(), id.clone()).await {
                        Some(mods) => {
                            return Some(LobbyReceipt {
                                header:header,
                                menu_items:receipt_items,
                                modifiers:mods
                            })
                        }
                        None => None
                    }
                },
                None => None
            }
        },
        None => None
   }
}

pub async fn get_claims(state:Data<AppState>, lobby_id:String) -> Option<Vec<ItemClaim>> {
    match sqlx::query_as::<_,ItemClaim>(
        "SELECT user_id, receipt_item_id AS item_id FROM item_claims WHERE lobby_id = $1"
    )
    .bind(lobby_id)
    .fetch_all(&state.db)
    .await {
        Ok(claims) => Some(claims),
        Err(err) => {
            println!("{:?}", err);
            None
        },
    }
}

pub async fn claim_item(state:Data<AppState>, claim:ItemClaim, lobby_id: String) -> Result<String, sqlx::Error> {
    match sqlx::query(
        "INSERT INTO item_claims(user_id, receipt_item_id, lobby_id) VALUES ($1, $2, $3)"
    )
    .bind(claim.user_id)
    .bind(claim.item_id)
    .bind(lobby_id.clone())
    .execute(&state.db)
    .await {
        Ok(_) => Ok(String::from("Okay")),
        Err(err) => Err(err)
    }
}

pub async fn unclaim_item(state:Data<AppState>, claim:ItemClaim) -> Result<String, sqlx::Error> {
    println!("{:}", claim.user_id.clone());
    match sqlx::query(
        "DELETE FROM item_claims WHERE user_id = $1 AND receipt_item_id = $2"
    )
    .bind(claim.user_id)
    .bind(claim.item_id)
    .execute(&state.db)
    .await {
        Ok(_) => Ok(String::from("Okay")),
        Err(err) => Err(err)
    }
}
#[derive(FromRow)]
pub struct LobbyCount {
    pub count: i64
}
pub async fn check_lobby(state:Data<AppState>, lobby_id:String) -> bool {
    match sqlx::query_as::<_, LobbyCount>(
        "SELECT COUNT(*) FROM lobbies WHERE lobby_id = $1"
    )
    .bind(lobby_id.clone())
    .fetch_one(&state.db)
    .await {
        Ok(count) => count.count > 0,
        Err(_) => false
    }
}

pub async fn delete_lobby(state:Data<AppState>, lobby_id:String) -> bool {
    match sqlx::query(
        "DELETE FROM lobbies WHERE lobby_id = $1"
    )
    .bind(lobby_id.clone())
    .execute(&state.db)
    .await {
        Ok(_) => true,
        Err(_) => false
    }
}