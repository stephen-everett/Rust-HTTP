use actix_web::{post, web::{Data, Json}, Responder, HttpResponse};
use serde::Deserialize;
use crate::structs::{app_state::AppState, user::UserSearch};

#[derive(Deserialize)]
pub struct SearchParam{
    message:String
}

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