use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MenuItem {
    pub sku: i32,
    pub name: String,
    pub quantity: i32
}