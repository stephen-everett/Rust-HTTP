use actix_web::{get, web::{Data, ReqData}, Responder, HttpResponse};
use crate::structs::app_state::{AppState, TokenClaims};

/// Deletes user by extracting user_id from ReqData claims (JWT Token)
/*
    Author: Stephen Everett
    Contributors: Khirby Calma(Front-end)
*/
#[get("/api/delete_user")]
async fn delete_user(state: Data<AppState>, claims: Option<ReqData<TokenClaims>>) -> impl Responder {
    
    match claims {
        Some(claims) => {
            let query = "DELETE FROM users WHERE user_id = $1";
            match sqlx::query(query)
                .bind(claims.user_id.to_string())
                .execute(&state.db)
                .await {
                    Ok(rows) => HttpResponse::Ok().json(format!("User has been deleted: {:?}", rows.rows_affected())),
                    Err(err) => HttpResponse::InternalServerError().json(format!("Something went wrong: {:?}", err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }

}