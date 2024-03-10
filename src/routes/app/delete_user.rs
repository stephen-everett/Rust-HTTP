use actix_web::{get, post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::bank_information::BankAccount;

/// Deletes user by extracting user_id from ReqData claims (JWT Token)
/*
    Author: Stephen Everett
    Contributors: Khirby Calma(Front-end)
*/
#[get("/delete_user")]
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


#[post("/delete_bank")]
async fn delete_bank(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<BankAccount>) -> impl Responder{
    match token {
        Some(token) => {
            let del_query = "DELETE FROM bank WHERE user_id = $1 and account_number = $2";
            match sqlx::query(del_query)
                .bind(token.user_id.to_string())
                .bind(body.bank_account.clone())
                .execute(&state.db)
                .await{
                   Ok(_)=> HttpResponse::Ok().json("bank removed"),
                   Err(_)=> HttpResponse::InternalServerError().json("bank not found")
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}