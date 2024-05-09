use std::{io::{Read, Write}, net::TcpStream};
use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::bank_information::BankInformation;
use std::str;

#[derive(serde::Serialize, serde::Deserialize)]
struct ResponseMessage{
    bank:BankInformation,
    amount:String
}

#[post("/charge_bank")]
async fn charge_bank(_state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<ResponseMessage>)-> impl Responder{
    match token {
        Some(token) => {
            let mut bank_connection = TcpStream::connect("127.0.0.1:8000").unwrap();
            let send_bank_charge = serde_json::to_string(&body).unwrap();
            let mut buf = String::new();
            let _ = bank_connection.write_all(send_bank_charge.as_bytes()).unwrap();
            // let response: usize = bank_connection.read(&mut [0;1024]).unwrap();
            let response = bank_connection.read_to_string(&mut buf).unwrap();
            let answer:String = str::from_utf8(&buf.as_bytes()).unwrap().parse().unwrap();

            
            tracing::info!("{:?}", response);
            tracing::info!("{:?} responsed from bank server", answer);
            // tracing::info!("{:?}", str::from_utf8(&buf.as_bytes()).unwrap());
            // let _ = bank_connection.write(send_bank_charge.as_bytes()).unwrap();
           

            if &answer == "OK" {
                tracing::info!("Success");
                HttpResponse::Ok().json("success")
            } else {
                tracing::error!("Error: {:?}", response);
                HttpResponse::InternalServerError().json("failure")
            }
            
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
        
    }

}