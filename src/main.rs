use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{fetch_messages, post_message, test_connection, clear_messages};

pub struct AppState {
    db: Pool<Postgres>
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
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(fetch_messages)
            .service(post_message)
            .service(test_connection)
            .service(clear_messages)
    })
    .bind(("0.0.0.0", 6000))?
    .run()
    .await
}