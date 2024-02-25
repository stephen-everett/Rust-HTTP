use actix_web::{post, web::{Data, ReqData}, Responder, HttpResponse};
use crate::structs::app_state::{AppState, TokenClaims};


#[post("/add_bank")]
async fn add_bank(state:Data<AppState>){

    
}

