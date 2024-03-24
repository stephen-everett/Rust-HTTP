use actix_web::web::Data;
use sqlx::FromRow;
use crate::{
    structs::{
        app_state::AppState, lobby::LobbyReceipt, receipt_item:: ReceiptItem
    },
    routes::app::post_receipt::{get_receipt_items, resolve_header, get_mods}
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