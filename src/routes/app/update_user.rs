use actix_web::{post, web::{Data, ReqData}, Responder, HttpResponse};
use crate::structs::app_state::{AppState, TokenClaims};





#[post("/update_first_name")]
async fn update_first_name(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{
match token{
    Some(token)=>{
        let upQuery = "UPDATE users SET first_name = $1 WHERE user_id = $2";
    }
}



}

#[post("/update_last_name")]
async fn update_last_name(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}


#[post("/update_phone_number")]
async fn update_phone_number(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}



#[post("update_email")]
async fn update_email(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}

#[post("/update_pin")]
async fn update_pin(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}

#[post("/update_password")]
async fn update_password(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}


