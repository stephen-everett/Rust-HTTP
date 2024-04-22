use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::user::{Password,PIN};
use argonautica::Verifier;

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
					let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
						.with_hash(password.password)   
						.with_password(&body.password)
						.with_secret_key(hash_secret)
						.verify()
						.unwrap();
					match is_valid {
						true => HttpResponse::Ok().json("✔️ Password Valid"),
						false => HttpResponse::Forbidden().json("❌ Password Invalid")
					}
                }, Err(err) => HttpResponse::BadRequest().json(err.to_string())
            }
		},None => HttpResponse::BadRequest().json("Problem with token")
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
						let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
						let mut verifier = Verifier::default();
						let is_valid = verifier
							.with_hash(pin.pin)
							.with_password(body.pin.clone())
							.with_secret_key(hash_secret)
							.verify()
							.unwrap();
						match is_valid {
							true => HttpResponse::Ok().json("✔️ Pin Valid"),
							false => HttpResponse::Forbidden().json("❌ Password Invalid")
						}
					}, Err(err) => HttpResponse::BadRequest().json(err.to_string())
				}
		},None => HttpResponse::BadRequest().json("Problem with token")
	}
}