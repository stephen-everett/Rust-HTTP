use actix_web::{post, web::{Data, ReqData,Json}, Responder, HttpResponse};
use crate::structs::{app_state::{AppState, TokenClaims}};
use crate::structs::bank_information::BankInformation;


#[post("/add_bank")]
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

// #[post("/add_bank")]
// async fn add_bank(state:Data<AppState>,token:Option<ReqData<TokenClaims>>,body:Json<BankInformation>)-> impl Responder{
//     match token{
//         Some(token) =>{
//             // let bank_stuff = *body.info;
//             let add_bank_query = "INSERT INTO bank(user_id,bank_name,routing_number,account_number)VALUES($1,$2,$3,$4)";
//             match sqlx::query(add_bank_query)
//                 .bind(token.user_id.to_string())
//                 .bind(body.bank_name.clone())
//                 .bind(body.bank_routing.clone())
//                 .bind(body.bank_account_number.clone())
//                 .execute(&state.db)
//             .await{
//              Ok(bank) => HttpResponse::Ok().json("bank added"),
//              Err(err) => HttpResponse::InternalServerError().json("bad")
//             }
//         },
//         None => HttpResponse::InternalServerError().json("Something was wrong with token")
//     }
    
// }