use actix_web::{get, web::Data, Responder, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Verifier;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::SignWithKey;
use crate::structs::{user::AuthUser, app_state::{AppState, TokenClaims}};

#[get("/login")]
async fn basic_auth(state:Data<AppState>, credentials:BasicAuth) -> impl Responder {
    let jwt_secret:Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET").expect("JWT_SECRET needs to be set!").as_bytes()).unwrap();

    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password!"),
        Some(pass) => {
            match sqlx::query_as::<_,AuthUser>(
                "SELECT users.user_id, username, password FROM users JOIN user_profiles USING (user_id) WHERE username = $1",
            )
            .bind(username.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(user) => {
                    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(user.password)
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();
                    
                    if is_valid {
                        let claims = TokenClaims {user_id: user.user_id};
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    }
                    else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                Err(err) =>{
                    match err {
                        sqlx::Error::RowNotFound => {
                            HttpResponse::Unauthorized().json("Incorrect username or password")
                        },
                        _ => HttpResponse::InternalServerError().json(format!("{:?}", err)),
                    }
                }
            }
        }
    }
    
}