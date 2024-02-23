use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::app_state::{AppState, TokenClaims};
use crate::structs::user::{FirstName,LastName,Password,PhoneNumber,PIN};




#[post("/update_first_name")]
async fn update_first_name(state:Data<AppState>,token: Option<ReqData<TokenClaims>>, body:Json<FirstName>) -> impl Responder{
match token {
    Some(token)=>{
        let name: FirstName = body.into_inner();
        let upQuery = "UPDATE users SET first_name = $1 WHERE user_id = $2";
        match sqlx::query(upQuery)
        .bind(name.name)
        .bind(token.user_id.to_string())
        .execute(&state.db)
        .await{
            Ok(name)=> HttpResponse::Ok().json("name has been changed"),
            Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
        }
     },
     None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[post("/update_last_name")]
async fn update_last_name(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}


#[post("/update_phone_number")]
async fn update_phone_number(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}



#[post("update_email")]
async fn update_email(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}

#[post("/update_pin")]
async fn update_pin(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}

#[post("/update_password")]
async fn update_password(state:Data<AppState>,token: Option<ReqData<TokenClaims>>) -> impl Responder{


    

}


