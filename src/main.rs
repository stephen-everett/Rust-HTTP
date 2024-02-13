// Third part Modules
use actix_web::{web, web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

// Our Modules
mod structs;
use structs::app_state::AppState;

mod routes;
use routes::{
    app::{search::search_user, delete_user::delete_user},
    auth::{register::create_user, login::basic_auth},
    debug::{get_all_users, test_connection, test_auth}
};

mod middleware;
use middleware::validator::validator;


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
                .service(web::scope("/api")
                    //.service(fetch_messages)
                    //.service(post_message)
                    //.service(test_connection)
                    //.service(clear_messages)
                    //.service(post_receipt)
                    //.service(join_lobby)
                    .service(create_user)
                    .service(basic_auth)
                        .service(web::scope("/debug")
                        .service(test_connection)
                        .service(get_all_users)
                        .service(
                            web::scope("")
                            .wrap(bearer_middleware.clone())
                            .service(test_auth)
                        )
                    )
                    .service(
                        web::scope("/app")
                        .wrap(bearer_middleware)
                        .service(search_user)
                        .service(delete_user)
                    )
                    //.service(search_user)
                    //.service(
                        //web::scope("")
                        //.wrap(bearer_middleware)
                        //.service(test_auth)
                        //.service(get_all_users)
                        //.service(delete_user)
                    //)
                    )
    })
    .keep_alive(std::time::Duration::from_secs(75)) // timeout set because of errors from Nginx. 75 seconds might be long
    .bind(("0.0.0.0", 6000))?
    .run()
    .await
}