use actix_web::{body, post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::{app_state::{AppState, TokenClaims},
                     user::{UpdatePIN,FirstName,LastName,UpdatePhoneNumber,UpdateEmail,UpdatePassword,Username}};
use argonautica::Hasher;
use crate::routes::auth::register::{unique_phone,unique_email,unique_username};


#[post("/username")]
async fn update_username(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<Username>)->impl Responder{
    match token{
        Some(token) => {
            if !unique_username(state.clone(), body.name.clone()).await{
                return  HttpResponse::Conflict().json("bad name");
            }
            else{
                let up_query = "UPDATE users SET username = $1 WHERE user_id = $2";
                match sqlx::query(up_query)
                .bind(body.name.clone())
                .bind(token.user_id.to_string())
                .execute(&state.db)
                .await{
                    Ok(user)=>HttpResponse::Ok().json("updated username"),
                    Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
            }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

#[post("/first_name")]
async fn update_first_name(state:Data<AppState>,token: Option<ReqData<TokenClaims>>, body:Json<FirstName>) -> impl Responder{
    match token {
    Some(token)=>{
        //let name: FirstName = body.into_inner();
        let up_query = "UPDATE users SET first_name = $1 WHERE user_id = $2";
        match sqlx::query(up_query)
        .bind(body.name.clone())
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

#[post("/last_name")]
async fn update_last_name(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<LastName>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            let up_query = "UPDATE users SET last_name = $1 WHERE user_id = $2";
            match sqlx::query(up_query)
            .bind(body.name.clone())
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

#[post("/phone_number")]
async fn update_phone_number(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<UpdatePhoneNumber>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            if !unique_phone(state.clone(),body.number.clone()).await{
                return  HttpResponse::Conflict().json("Not a unique Phone number");
            }
            else{let up_query = "UPDATE users SET phone_number = $1 WHERE user_id = $2";
                match sqlx::query(up_query)
                .bind(body.number.clone())
                .bind(token.user_id.to_string())
                .execute(&state.db)
                .await{
                    Ok(name)=> HttpResponse::Ok().json("name has been changed"),
                    Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
            }
         },
         None => HttpResponse::InternalServerError().json("Something was wrong with token")
        }
}

#[post("/email")]
async fn update_email(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<UpdateEmail>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            if !unique_email(state.clone(), body.name.clone()).await{
                return HttpResponse::Conflict().json("not unique email");
            }
            else{
                let up_query = "UPDATE users SET email = $1 WHERE user_id = $2";
                match sqlx::query(up_query)
                .bind(body.name.clone())
                .bind(token.user_id.to_string())
                .execute(&state.db)
                .await{
                    Ok(name)=> HttpResponse::Ok().json("name has been changed"),
                    Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
            }
          
         },
         None => HttpResponse::InternalServerError().json("Something was wrong with token")
        }
}

#[post("/pin")]
async fn update_pin(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<UpdatePIN>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            let up_query = "UPDATE users SET pin = $1 WHERE user_id = $2";
            match sqlx::query(up_query)
            .bind(body.pin.clone())
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

#[post("/password")]
async fn update_password(state:Data<AppState>,token: Option<ReqData<TokenClaims>>, body:Json<UpdatePassword>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            let up_query = "UPDATE users SET password = $1 WHERE user_id = $2";
            let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET needs to be set!");
            let mut hasher = Hasher::default();
            let hash = hasher
            .with_password(body.pass.clone())
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();
            
            match sqlx::query(up_query)
            .bind(hash)
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