use serde::{Serialize, Deserialize};
use sqlx::FromRow;

/// Struct to get the bank information for the user to send it to the server.
/// userid is the primary key to find the the bank information on the user
/// Author: Luis Baca
#[derive(Serialize,Deserialize,FromRow)]
struct BankInformation{
    user_id: String,
    bank_name: String,
    bank_routing: String,
    bank_account_number:String
}