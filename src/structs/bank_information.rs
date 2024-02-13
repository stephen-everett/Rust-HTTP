use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize,Deserialize,FromRow)]
struct BankInformation{
    bank_name: String,
    bank_routing: String,
    bank_account_number:String
}