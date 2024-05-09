use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::structs::receipt_item::ReceiptItem;

#[derive(Serialize, Deserialize)]
pub struct Lobby {
    pub lobby_id: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResturauntMenuItem {
    pub sku: String,
    pub name: String,
    pub price: i64,
    pub modifiers:Vec<ItemModifier>
}

#[derive(Serialize, Deserialize)]
pub struct ResturauntReceipt {
    pub res_id: String,
    pub menu_items: Vec<ResturauntMenuItem>
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct ItemModifier {
    pub name: String,
    pub price: i64,
    pub receipt_item_id: Option<String>
}

#[derive(Serialize, Debug, Clone)]
pub struct LobbyReceipt {
    pub header: StateHeader,
    pub menu_items: Vec<ReceiptItem>,
    pub modifiers: Vec<ItemModifier>
}

#[derive(FromRow, Serialize, Debug,Clone)]
pub struct StateHeader {
    pub res_id: String,
    pub lobby_id: String,
    pub receipt_id: String
}

#[derive(Deserialize)]
pub struct UpdateItem {
    pub item_id: String,
    pub lobby_id: String
}

#[derive(Deserialize)]
pub struct LobbyInvite {
    pub friend_id: String,
    pub lobby_id: String
}

#[derive(Serialize, FromRow)]
pub struct IncomingLobbyInvite {
    pub lobby_id: String,
    pub friend_id: String,
}
