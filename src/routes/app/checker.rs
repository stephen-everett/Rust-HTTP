use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::user::{Password,PIN};
use argonautica::{Hasher, Verifier};

// MARK: Password
#[post("/password")]
async fn is_password(state: Data<AppState>, token: Option<ReqData<TokenClaims>>, body: Json<Password>) -> impl Responder {
    match token {
    Some(token) => {
		let get_password = "SELECT password FROM users WHERE user_id = $1";
		match sqlx::query_as::<_, Password>(get_password)
            .bind(token.user_id.to_string())
            .fetch_one(&state.db)
            .await {
                Ok(password) => {
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
						.with_hash(password.pass)   
						.with_password(body.pass.clone())
						.with_secret_key("HASH_SECRET")
						.verify()
						.unwrap();
					match is_valid {
						true => HttpResponse::Ok(),
						false => HttpResponse::BadRequest()
					}
                }, Err(_) => HttpResponse::BadRequest()
            }
		},None => HttpResponse::BadRequest()
	}
}


// MARK: PIN
#[post("/pin")]
async fn is_pin(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<PIN>)-> impl Responder{
	match token {
		Some(token) => {
			let get_pin = "SELECT pin FROM users WHERE user_id = $1";
			match sqlx::query_as::<_, PIN>(get_pin)
				.bind(token.user_id.to_string())
				.fetch_one(&state.db)
				.await {
					Ok(pin) => {
						let mut verifier = Verifier::default();
						let is_valid = verifier
							.with_hash(pin.pin)
							.with_password(body.pin.clone())
							.with_secret_key("HASH_SECRET")
							.verify()
							.unwrap();
						match is_valid {
							true => HttpResponse::Ok(),
							false => HttpResponse::BadRequest()
						}
					}, Err(_) => HttpResponse::BadRequest()
				}
		},None => HttpResponse::BadRequest()
	}
}