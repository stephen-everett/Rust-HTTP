use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct ReceiptItem {
    pub lobby_id: i32,
    pub sku: i32,
    pub name: String,
    pub quantity: i32
}