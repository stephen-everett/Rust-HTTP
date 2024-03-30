use actix_web::{ post, web::{Data, Json, ReqData}, HttpResponse, Responder};
use crate::structs::{app_state::{AppState, TokenClaims},
                     user::{PIN,FirstName,LastName,PhoneNumber,
                            Email,Password,Username,
                            Picture}};
use argonautica::Hasher;
use crate::routes::auth::register::{unique_phone,unique_email,unique_username};

/// The user sends a new username to be d on the database. The function is going to check if the username is already
/// in the database. If it is not in the database, then the username would be changed in database if it is available.
#[post("/username")]
async fn update_username(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<Username>)->impl Responder{
    match token{
        Some(token) => {
            if !unique_username(state.clone(), body.name.clone()).await{
                return  HttpResponse::Conflict().json("bad name");
            }
            else{
                let up_query = "UPDATE user_profiles SET username = $1                                     
                                      WHERE user_id = $2";
                match sqlx::query(up_query)
                    .bind(body.name.clone())
                    .bind(token.user_id.to_string())
                    .execute(&state.db)
                .await{
                    Ok(_)=>HttpResponse::Ok().json("d username"),
                    Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
            }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

/// sets the first_name of the user
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
            Ok(_)=> HttpResponse::Ok().json("First name has been changed"),
            Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
            }
         },
     None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

/// Changes the last name of the user
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
                Ok(_)=> HttpResponse::Ok().json("last name has been changed"),
                Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
            }
         },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
        }
}

/// sets the phone_number of the user and checks if someone else already has the phone_number
#[post("/phone_number")]
async fn update_phone_number(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<PhoneNumber>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            if !unique_phone(state.clone(),body.number.clone()).await{
                return  HttpResponse::Conflict().json("Not a vaild Phone number");
            }
            else{let up_query = "UPDATE users SET phone_number = $1 WHERE user_id = $2";
                match sqlx::query(up_query)
                    .bind(body.number.clone())
                    .bind(token.user_id.to_string())
                    .execute(&state.db)
                .await{
                    Ok(_)=> HttpResponse::Ok().json("Phone number has been changed"),
                    Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
            }
         },
         None => HttpResponse::InternalServerError().json("Something was wrong with token")
        }
}

/// ses the email for the user and checks if the email is unique and free
#[post("/email")]
async fn update_email(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<Email>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            if !unique_email(state.clone(), body.name.clone()).await{
                return HttpResponse::Conflict().json("not a vaild email");
            }
            else{
                let up_query = "UPDATE users SET email_address = $1 WHERE user_id = $2";
                match sqlx::query(up_query)
                    .bind(body.name.clone())
                    .bind(token.user_id.to_string())
                    .execute(&state.db)
                .await{
                    Ok(_)=> HttpResponse::Ok().json("email has been changed"),
                    Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
            }
         },
         None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

/// set the PIN 
#[post("/pin")]
async fn update_pin(state:Data<AppState>,token: Option<ReqData<TokenClaims>>,body:Json<PIN>) -> impl Responder{
    match token {
        Some(token)=>{
            //let name: FirstName = body.into_inner();
            let mut hasher = Hasher::default();
            let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET needs to be set!");
            let pin_hash = hasher.with_password(body.pin.clone()).with_secret_key(hash_secret).hash().unwrap();
            let up_query = " UPDATE users SET pin = $1 WHERE user_id = $2";
            match sqlx::query(up_query)
                .bind(pin_hash)
                .bind(token.user_id.to_string())
                .execute(&state.db)
            .await{
                Ok(_)=> HttpResponse::Ok().json("PIN has been changed"),
                Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
            }
         },
         None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}

/// the user sends a password where it is going to be hashed to be stored in database
#[post("/password")]
async fn update_password(state:Data<AppState>,token: Option<ReqData<TokenClaims>>, body:Json<Password>) -> impl Responder{
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
                Ok(_)=> HttpResponse::Ok().json("Password has been changed"),
                Err(err)=> HttpResponse::InternalServerError().json(format!("{:?}",err))
            }
         },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}


/// changes the current picture to a new one
#[post("/picture")]
async fn update_picture(state: Data<AppState>, token: Option<ReqData<TokenClaims>>, body:Json<Picture>)-> impl Responder{
    match token{
        Some(token) => {
           let pic_q = "UPDATE profile_pictures SET picture = $1 WHERE user_id = $2";
           match sqlx::query(pic_q) 
               .bind(body.picture.clone())
               .bind(token.user_id.to_string())
               .execute(&state.db)
               .await{
                Ok(_) => HttpResponse::Ok().json("picture changed"),
                Err(e)=> HttpResponse::InternalServerError().json(format!("{:?}",e))
               }
        },
        None => HttpResponse::InternalServerError().json("Something was wrong with token")
    }
}