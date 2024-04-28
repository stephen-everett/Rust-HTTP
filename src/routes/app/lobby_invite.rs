
/*
    Author: Stephen Everett
    The endpoints used for lobby invites
 */

 use actix_web::{get, post, web::{Data, ReqData, Json}, Responder, HttpResponse};
 use crate::structs::{
     app_state::{AppState, TokenClaims},
     lobby::{LobbyInvite, Lobby, IncomingLobbyInvite}
 };
 

#[post("/send_invite")]
async fn send_lobby_invite(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>, body: Json<LobbyInvite>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query("INSERT INTO lobby_invites(user_id, from_id, lobby_id) VALUES($1,$2,$3)")
                .bind(&body.friend_id)
                .bind(&claims.user_id.to_string())
                .bind(&body.lobby_id)
                .execute(&state.db)
                .await {
                    Ok(_) => HttpResponse::Ok().json("Request Sent".to_string()),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[post("/cancel_invite")]
async fn cancel_invite(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>, body: Json<LobbyInvite>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query("DELETE FROM lobby_invites WHERE from_id = $1 AND user_id = $2 AND lobby_id = $3")
                .bind(&claims.user_id.to_string())
                .bind(&body.friend_id)
                .bind(&body.lobby_id)
                .execute(&state.db)
                .await {
                    Ok(_) => HttpResponse::Ok().json("Request Canceled".to_string()),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[post("/remove_invite")]
async fn remove_invite(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>, body: Json<Lobby>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query("DELETE FROM lobby_invites WHERE user_id = $1 AND lobby_id = $2")
                .bind(&claims.user_id.to_string())
                .bind(&body.lobby_id)
                .execute(&state.db)
                .await {
                    Ok(_) => HttpResponse::Ok().json("Invite Removed".to_string()),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[post("/decline_invite")]
async fn decline_invite(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>, body: Json<LobbyInvite>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query("DELETE FROM lobby_invites WHERE user_id = $1 AND from_id = $2 AND lobby_id = $3")
                .bind(&claims.user_id.to_string())
                .bind(&body.friend_id)
                .bind(&body.lobby_id)
                .execute(&state.db)
                .await {
                    Ok(_) => HttpResponse::Ok().json("Invite Decliend".to_string()),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[get("/get_incoming")]
async fn get_incoming_invites(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder {
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

#[get("/get_outgoing")]
async fn get_outgoing_invites(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query_as::<_,IncomingLobbyInvite>("SELECT lobby_id, from_id AS friend_id FROM lobby_invites WHERE from_id = $1")
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