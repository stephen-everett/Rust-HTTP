/*
    Endpoint used for checkout
 */

use actix_web::{get, post, web::{Data, ReqData, Json}, Responder, HttpResponse};
use crate::structs::{
    app_state::{AppState, TokenClaims},
    lobby::{LobbyInvite, Lobby, IncomingLobbyInvite}
};

#[get("/checkout")]
async fn checkout(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query_as::<_,IncomingLobbyInvite>("SELECT lobby_id, from_id AS friend_id FROM lobby_invites WHERE user_id = $1")
                .bind(&claims.user_id.to_string())
                .fetch_all(&state.db)
                .await {
                    Ok(invites) => HttpResponse::Ok().json(invites),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}