use actix_web::{post, web::{Data, Json}, Responder, HttpResponse};
use crate::structs::app_state::AppState;
use crate::structs::user::Picture;
use crate::websockets::messages::server_message::DUser;

/// returns the profile picture of a given user using their user_id
#[post("/get_pic")]
async fn user_pic(state: Data<AppState>, body: Json<DUser>) -> impl Responder{

    let query = "SELECT picture FROM profile_pictures WHERE user_id = $1 ";
    match sqlx::query_as::<_,Picture>(
        query
    )
        .bind(body.user_id.to_string())
        .fetch_one(&state.db)
        .await {
            Ok(pic) => HttpResponse::Ok().json(pic),
            Err(err) => HttpResponse::InternalServerError().json(format!("Something went wrong: {:?}", err))
        }
}
