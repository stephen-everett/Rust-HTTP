use crate::structs::app_state::AppState;
use actix_web::{post, web::Data, HttpResponse, Responder};
use argonautica::Hasher;
use sqlx::prelude::FromRow;
#[derive(FromRow)]
struct ChangePins {
    user_id: String,
    pin: String,
}
/// Force server to change all current pins to hashed values
#[post("/hashpins")]
async fn hash_pins(state: Data<AppState>) -> impl Responder {
    let get_pins = "SELECT user_id, pin FROM users";
    match sqlx::query_as::<_, ChangePins>(get_pins)
        .fetch_all(&state.db)
        .await
    {
        Ok(pin) => {
            for current_pin in pin.iter() {
                let hash_secret =
                    std::env::var("HASH_SECRET").expect("HASH_SECRET needs to be set!");
                let mut hasher = Hasher::default();
                let hash = hasher
                    .with_password(current_pin.pin.clone())
                    .with_secret_key(hash_secret)
                    .hash()
                    .unwrap();

                let up_current = "UPDATE users SET pin = $1 WHERE user_id = $2";
                match sqlx::query(up_current)
                    .bind(hash)
                    .bind(current_pin.user_id.clone())
                    .execute(&state.db)
                    .await
                {
                    Ok(_) => (),
                    Err(err) => return HttpResponse::BadRequest().json(err.to_string()),
                };
            }
            HttpResponse::Ok().json("Pins hashed")
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}