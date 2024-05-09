use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct ReceiptItem {
    pub receipt_item_id: String,
    pub sku: String,
    pub name: String,
    pub price: i64
}

#[derive(Serialize, FromRow)]
pub struct ItemModifier {
    pub receipt_item_id: String,
    pub modifier_name: String,
    pub modifier_price: i64
}