use serde::{Serialize, Deserialize};
use num::BigInt;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemModifier {
    pub name: String,
    pub price: i64,
    pub receipt_item_id: Option<String>
}
