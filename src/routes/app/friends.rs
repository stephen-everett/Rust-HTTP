
use actix_web::{get, post, web::{Data, ReqData, Json}, Responder, HttpResponse};
use crate::structs::{
    app_state::{AppState, TokenClaims},
    friends_list::{Friend, RequestId}
};

#[post("/send_request")]
async fn send_friend_request(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>, body: Json<RequestId>) -> impl Responder {

    match claims {
        Some(claims) => {
            match sqlx::query("INSERT INTO friends_list(user_id, friend_id, status) VALUES ($1,$2,$3), ($4,$5,$6)")
                .bind(claims.user_id.clone().to_string())
                .bind(body.user_id.clone())
                .bind("Outgoing".to_string())
                .bind(body.user_id.clone())
                .bind(claims.user_id.clone().to_string())
                .bind("Incoming".to_string())
                .execute(&state.db)
                .await {
                    Ok(_) => HttpResponse::Ok().json("Request Sent".to_string()),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[post("/accept_request")]
async fn accept_friend_request(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>, body: Json<RequestId>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query("UPDATE friends_list SET status = 'Accepted' WHERE (user_id = $1 AND friend_id = $2) OR (user_id = $2 AND friend_id = $1)")
                .bind(claims.user_id.clone().to_string())
                .bind(body.user_id.clone())
                .execute(&state.db)
                .await {
                    Ok(_) => HttpResponse::Ok().json("Friend Accepted".to_string()),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[post("/deny_request")]
async fn deny_friends_request(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>, body: Json<RequestId>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query("DELETE FROM friends_list WHERE (user_id = $1 AND friend_id = $2) OR (user_id = $2 AND friend_id = $1)")
                .bind(claims.user_id.clone().to_string())
                .bind(body.user_id.clone())
                .execute(&state.db)
                .await {
                    Ok(_) => HttpResponse::Ok().json("Friend Denied".to_string()),
                    Err(err) => HttpResponse::Conflict().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[get("/accepted_friends")]
async fn get_accepted_friends(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query_as::<_,Friend>("
            SELECT friend_id, username, status FROM (
                SELECT DISTINCT friend_id AS f_id, username
                FROM friends_list JOIN user_profiles on (friend_id = user_profiles.user_id)
            ) AS name_resolve
            JOIN friends_list ON(friend_id = f_id)
            WHERE user_id = $1 AND status = \'Accepted\'")
                .bind(claims.user_id.clone().to_string())
                .fetch_all(&state.db)
                .await {
                    Ok(friends_list) => HttpResponse::Ok().json(friends_list),
                    Err(err) => HttpResponse::InternalServerError().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[get("/outgoing_friends")]
async fn get_outgoing_friends(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query_as::<_,Friend>("
            SELECT friend_id, username, status FROM (
                SELECT DISTINCT friend_id AS f_id, username
                FROM friends_list JOIN user_profiles on (friend_id = user_profiles.user_id)
            ) AS name_resolve
            JOIN friends_list ON(friend_id = f_id)
            WHERE user_id = $1 AND status = \'Outgoing\'")
                .bind(claims.user_id.clone().to_string())
                .fetch_all(&state.db)
                .await {
                    Ok(friends_list) => HttpResponse::Ok().json(friends_list),
                    Err(err) => HttpResponse::InternalServerError().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[get("/incoming_friends")]
async fn get_incoming_friends(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder {
    match claims {
        Some(claims) => {
            match sqlx::query_as::<_,Friend>("
            SELECT friend_id, username, status FROM (
                SELECT DISTINCT friend_id AS f_id, username
                FROM friends_list JOIN user_profiles on (friend_id = user_profiles.user_id)
            ) AS name_resolve
            JOIN friends_list ON(friend_id = f_id)
            WHERE user_id = $1 AND status = \'Incoming\'")
                .bind(claims.user_id.clone().to_string())
                .fetch_all(&state.db)
                .await {
                    Ok(friends_list) => HttpResponse::Ok().json(friends_list),
                    Err(err) => HttpResponse::InternalServerError().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}