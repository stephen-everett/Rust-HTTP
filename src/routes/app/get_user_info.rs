use actix_web::{get, web::{Data, ReqData}, Responder, HttpResponse};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::user::UserNoPassword;


/// gets all the user information except pin and password
#[get("/get_info")]
async fn user_info(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder{

    match claims {
        Some(claims) => {
            let query = "SELECT user_id, username, first_name, last_name, email_address, phone_number, birthdate FROM users JOIN user_profiles USING(user_id) WHERE user_id = $1 ";
            match sqlx::query_as::<_,UserNoPassword>(
                query
            )
                .bind(claims.user_id.to_string())
                .fetch_all(&state.db)
                .await {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}