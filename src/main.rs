use actix_web::{web::Data, App, HttpServer, dev::ServiceRequest, error::Error, HttpMessage};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{fetch_messages, post_message, test_connection, clear_messages, post_receipt, join_lobby,create_user, basic_auth,search_user};


use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

extern crate argonautica;

pub struct testVariable {
    id:i32,
}

pub struct AppState {
    db: Pool<Postgres>
}

// structure for bearer token. Can contain more information (such as permissions)
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id:i32,
}

// middleware to validate token
async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // create key using JWT_SECRET environment variable
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    
    // grab token from credentials passed from request
    let token_string = credentials.token();

    // check to see if token  is valid
    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid Token");

    // check claims. If the token is valid, pass it on to the route. If not return error
    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        }

    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
        Create a connection pool to the PG database using env variable "DATABASE_URL". In dev environment
        this is defined in a .env file in the root folder of the project. In production it's defined in an
        actual environment variable. (Don't upload .env to github. Add to .gitignore)
     */
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");


    /*
        Start the server and use defined endpoints. Endpoints are defined in services.rs
        and imported at the top of this file
     */
    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(fetch_messages)
            .service(post_message)
            .service(test_connection)
            .service(clear_messages)
            .service(post_receipt)
            .service(join_lobby)
            .service(create_user)
            .service(basic_auth)
            .service(search_user)
    })
    .bind(("0.0.0.0", 6000))?
    .run()
    .await
}