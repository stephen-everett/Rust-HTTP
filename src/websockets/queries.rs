use actix_web::web::Data;
use sqlx::FromRow;
use crate::structs::{
    app_state::AppState,
    receipt_item::ReceiptItem
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

pub async fn get_receipt(state:Data<AppState>, id:String) -> Vec<ReceiptItem>{
    // get search parameter from body
    // query
    match sqlx::query_as::<_,ReceiptItem>(
        "SELECT lobby_id, sku, name, quantity FROM receipt_item JOIN lobby USING(lobby_id) JOIN menu_item USING(sku) WHERE lobby_id = $1",
    )
    .bind(id.parse::<i32>().unwrap())
    .fetch_all(&state.db)
    .await
    {
        Ok(data) => data,
        Err(_) => Vec::new()
    }
}