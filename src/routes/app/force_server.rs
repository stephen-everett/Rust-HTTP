use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::AppState;
use crate::argonautica::Hasher;
use crate::structs::PIN;


async fn hash_pins(state:Data<AppState>){

}