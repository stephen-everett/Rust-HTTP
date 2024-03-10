use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};

use serde::Deserialize;
use crate::structs::{app_state::{AppState, TokenClaims}, bank_information::BankInformation, user::UserSearch};

#[derive(Deserialize)]
pub struct SearchParam{
    message:String
}
/// Basic search using the user's id. Responds with user information. 
/*
    Authors:  Luis Baca
 */
#[post("/search")]
pub async fn search_user(state:Data<AppState>,body:Json<SearchParam>) -> impl Responder {
    // get search parameter from body
    let search_param: SearchParam = body.into_inner();

    let search_query = format!("SELECT users.user_id, username, first_name, last_name FROM user_profiles JOIN users USING(user_id) WHERE LOWER(username) LIKE \'%{}%\'", search_param.message.to_lowercase());
    let search_query = search_query.as_str();
    // query
    match sqlx::query_as::<_,UserSearch>(
    search_query
    )
    //.bind(search_param.message.to_lowercase())
    .fetch_all(&state.db)
    .await
    {
        //Ok(UserSearch) => HttpResponse::Ok().json(UserSearch),
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("User not found") 
    }
}
/// searches the database with the likely hood of having the same first name and grabs all like users
#[post("/search_fname")]
pub async fn search_user_fname(state:Data<AppState>,body:Json<SearchParam>) -> impl Responder {
    // get search parameter from body
    let search_param: SearchParam = body.into_inner();

    let search_query = format!("SELECT users.user_id, username, first_name, last_name FROM user_profiles JOIN users USING(user_id) WHERE LOWER(first_name) LIKE \'%{}%\'", search_param.message.to_lowercase());
    let search_query = search_query.as_str();
    // query
    match sqlx::query_as::<_,UserSearch>(
    search_query
    )
    //.bind(search_param.message.to_lowercase())
    .fetch_all(&state.db)
    .await
    {
        //Ok(UserSearch) => HttpResponse::Ok().json(UserSearch),
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("User not found") 
    }
}
/// searches the database with the likely hood of having the same last name and grabs all like users
#[post("/search_lname")]
pub async fn search_user_lname(state:Data<AppState>,body:Json<SearchParam>) -> impl Responder {
    // get search parameter from body
    let search_param: SearchParam = body.into_inner();

    let search_query = format!("SELECT users.user_id, username, first_name, last_name FROM user_profiles JOIN users USING(user_id) WHERE LOWER(last_name) LIKE \'%{}%\'", search_param.message.to_lowercase());
    let search_query = search_query.as_str();
    // query
    match sqlx::query_as::<_,UserSearch>(
    search_query
    )
    //.bind(search_param.message.to_lowercase())
    .fetch_all(&state.db)
    .await
    {
        //Ok(UserSearch) => HttpResponse::Ok().json(UserSearch),
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("User not found") 
    }
}
/// searches all the users banks and returns them in a vector of all possible matches
#[post("/search_user_banks")]
async fn search_user_bank(state:Data<AppState>, token:Option<ReqData<TokenClaims>>)->impl Responder{
    match token{
        Some(token) =>{
            let search_bank = "SELECT * FROM banks WHERE user_id = $1";
            match sqlx::query_as::<_,BankInformation>(search_bank)
                .bind(token.user_id.to_string())
                .fetch_all(&state.db)
                .await{
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(err) => HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}