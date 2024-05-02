use crate::structs::app_state::AppState;
use crate::structs::bank_information::BankInformation;
use actix::dev::channel::AddressSender;
use actix_web::{post, web::Data, Responder, HttpResponse};



#[post("/call_bank")]
async fn call_bank(state:Data<AppState>,body:BankInformation) -> impl Responder{

    let res = AddressSender::new(&body).send(&state).await;
    HttpResponse::Ok().json(res)

}
