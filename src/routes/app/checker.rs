use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::user::{Password,PIN};
use argonautica::{Hasher, Verifier};

#[post("/password")]
async fn is_password(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<Password>) -> impl Responder{
    match token {
        Some(token) => {
            let pas_q = "SELECT password FROM users WHERE user_id = $1";
            match sqlx::query_as::<_,Password>(pas_q)
                .bind(token.user_id.to_string())
                .fetch_one(&state.db)
                .await{
                    Ok(password) => {
                        // let incoming_password = body.pass.clone();
                        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                        let mut hasher = Hasher::default();
                        let hash = hasher
                                            .with_password(body.pass.clone())
                                            .with_secret_key(hash_secret)
                                            .hash()
                                            .unwrap();
                        let incoming_password = Password{ pass: hash};
                        match password == incoming_password{
                        // match password == incoming_password.pass {
                            //true => HttpResponse::Ok().status(StatusCode::OK),
                            true => HttpResponse::Ok(),
                            //false => HttpResponse::InternalServerError().status(StatusCode::BAD_REQUEST),
                            false => HttpResponse::Forbidden()
                        }

                    },
                    Err(_err)=>{
                        //HttpResponse::InternalServerError().status(StatusCode::BAD_REQUEST)
                        HttpResponse::BadRequest()
                    }
                }
        }, 
        None => HttpResponse::BadRequest() 
    }

}


#[post("/pin")]
async fn is_pin(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<PIN>)-> impl Responder{
    match token {
        Some(token) =>{
            let get_pin = "SELECT pin FROM users WHERE user_id = $1";
            match sqlx::query_as::<_,PIN>(get_pin)
                .bind(token.user_id.to_string())
                .fetch_one(&state.db)
                .await{
                    Ok(pin)=>{
                        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                        // let mut hasher = Hasher::default();
                        // let hash = hasher
                        //     .with_password(body.pass.clone())
                        //     .with_secret_key(hash_secret)
                        //     .hash()
                        //     .unwrap();
                        // let incoming_pin = PIN{ pin: hash};
                        let mut verifier = Verifier::default();
                        let is_valid = verifier
                                                .with_hash(pin.pin)
                                                .with_password(body.pin.clone())
                                                .with_secret_key(hash_secret)
                                                .verify()
                                                .unwrap();
                        // if is_valid{
                        //     HttpResponse::Ok()
                        // }
                        // else{
                        //     HttpResponse::BadRequest()
                        // }
                        match is_valid{
                            true => HttpResponse::Ok(),
                            false => HttpResponse::Forbidden()
                        }
                        
                    },Err(_) => HttpResponse::BadRequest()
                }
            
        },
        None => HttpResponse::BadRequest()

    }
}