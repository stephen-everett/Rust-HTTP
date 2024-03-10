use actix_web::{body, post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::{app_state::{AppState, TokenClaims}};
use crate::structs::bank_information::BankInformation;
use crate::structs::user::Picture;

/// adds a new bank to the user profile
#[post("/bank")]
async fn add_bank(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<BankInformation>)-> impl Responder{
    match token{
        Some(token) =>{
            // let bank_stuff = *body.info;
            let add_bank_query = "INSERT INTO bank(user_id,bank_name,routing_number,account_number)VALUES($1,$2,$3,$4)";
            match sqlx::query(add_bank_query)
                .bind(token.user_id.to_string())
                .bind(body.bank_name.clone())
                .bind(body.routing_number.clone())
                .bind(body.account_number.clone())
                .execute(&state.db)
            .await{
                Ok(bank) => HttpResponse::Ok().json("bank added"),
                Err(err) => HttpResponse::InternalServerError().json("Failure to add a bank")
            }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }  
}

/// stores a picture in a HEX format to save on space
#[post("/picture")]
async fn add_picture(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<Picture>)-> impl Responder{
    match token {
        Some(token) => {
            let add_pic = "INSERT INTO profile_pictures(user_id, picture) VALUES($1,$2)";
            match sqlx::query(add_pic)
                .bind(token.user_id.to_string())
                .bind(body.picture.clone())
                .execute(&state.db)
                .await{
                    Ok(_) => HttpResponse::Ok().json("picture added"),
                    Err(e)=>HttpResponse::InternalServerError().json(format!("{:?}",e))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token") 
    }
}
