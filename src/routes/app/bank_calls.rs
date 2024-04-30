use std::net::TcpStream;
use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::bank_information::BankInformation;

struct ResponseMessage{
    header:String
}

async fn charge_bank(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:BankInformation)-> impl Responder{
    match token {
        Some(token) => {
            let mut bank_connection = TcpStream::connect("127.0.0.1:8000").unwrap();
            let send_bank_charge = serde_json::to_string(&body).unwrap();
            let _ = bank_connection.write_all(send_bank_charge.as_bytes())?;
            let response = bank_connection.read_to_string().unwrap();

            if response == "200"{
                HttpResponse::Ok().json("success")
            } else {
                HttpResponse::InternalServerError().json("failure")
            }
            
        }
        
    }

}