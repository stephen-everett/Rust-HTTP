use serde::{Serialize, Deserialize};
use sqlx::FromRow;
// use crate::structs::user::User;

/// Struct to get the bank information for the user to send it to the server.
/// userid is the primary key to find the the bank information on the user
/// Author: Luis Baca
#[derive(Serialize,Deserialize,FromRow)]
pub struct BankInformation{
    // pub user_id: String,
    pub bank_name: String,
    pub routing_number: String,
    pub account_number:String
}

#[derive(Deserialize)]
pub struct BankAccount{
    pub bank_account: String
}
// #[derive(Serialize,Deserialize,FromRow)]
// pub struct UserBank{
//     pub info: BankInformation
// }

// impl std::ops::Deref for UserBank{
//     type Target = BankInformation;
//     fn deref(&self) -> &Self::Target {
//         &self.info
//     }
// }
