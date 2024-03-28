/*
   Third Party Module
*/
// actix
use actix_web::{web, web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

// actix websockets
use actix::Actor;

// load environment variables and PgPool
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

/*
   Our Modules
*/
// AppState to store state
//mod structs;
use hello_rocket::structs::app_state::AppState;

// API Endpoints
//mod routes;
use hello_rocket::routes::{
    app::{
        add::{add_bank, add_picture},
        delete_user::{delete_bank, delete_user},
        friends::{
            accept_friend_request, deny_friends_request, get_accepted_friends,
            get_incoming_friends, get_outgoing_friends, send_friend_request,
        },
        get_user_info::{other_user, user_info},
        post_receipt::{get_receipt, post_receipt, delete_item},
        search::{search_user, search_user_bank, search_user_fname, search_user_lname},
        update_user::{
            update_email, update_first_name, update_last_name, update_password,
            update_phone_number, update_picture, update_pin, update_username,
        },
        get_profile_pic::user_pic,
        force_server::hash_pins,
        checker::{is_password,is_pin},
    },
    auth::{login::basic_auth, register::create_user},
    debug::{get_all_users, test_auth, test_connection},
    ws::ws::index,
};

// Validate JWT (Authentication)
use hello_rocket::middleware::validator::validator;
use hello_rocket::websockets::actors::waiting_room::WaitingRoom;

// Main function to start the server and provide access to endpoints
// Authors: Stephen Everett, Luis Baca
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
       Start the server and use defined endpoints. Endpoints are defined in routes folder
       and imported at the top of this file
    */

    let server = WaitingRoom::new(Data::new(AppState { db: pool.clone(), ws_server: None })).start();
    HttpServer::new(move || {
        // bearer middleware used to verify JWT token on protected routes.
        let bearer_middleware = HttpAuthentication::bearer(validator);

        App::new()
            .app_data(Data::new(AppState { db: pool.clone(), ws_server: Some(server.clone()) }))
            .service(
                web::scope("/api")
                    .service(hash_pins)
                    .service(create_user)
                    .service(basic_auth)
                    .service(
                        web::scope("/debug")
                            .service(test_connection)
                            .service(get_all_users)
                            .service(
                                web::scope("")
                                    .wrap(bearer_middleware.clone())
                                    .service(test_auth),
                            ),
                    )
                    .service(
                        web::scope("/pos")
                            .service(post_receipt)
                            .service(get_receipt)
                            .service(delete_item)
                        )
                    .service(
                        web::scope("/app")
                            .wrap(bearer_middleware)
                            .service(delete_user)
                            .service(user_info)
                            .service(delete_bank)
                            .service(other_user)
                            .service(user_pic)
                            .service(web::scope("/check")
                                         .service(is_pin)
                                         .service(is_password),
                                )
                            .service(
                                web::scope("/search")
                                    .service(search_user)
                                    .service(search_user_bank)
                                    .service(search_user_fname)
                                    .service(search_user_lname),
                            )
                            .service(web::scope("/add").service(add_picture).service(add_bank))
                            .service(
                                web::scope("/update")
                                    .service(update_first_name)
                                    .service(update_last_name)
                                    .service(update_email)
                                    .service(update_password)
                                    .service(update_username)
                                    .service(update_pin)
                                    .service(update_phone_number)
                                    .service(update_picture),
                            )
                            .service(
                                web::scope("/friends")
                                    .service(send_friend_request)
                                    .service(accept_friend_request)
                                    .service(get_accepted_friends)
                                    .service(get_incoming_friends)
                                    .service(get_outgoing_friends)
                                    .service(deny_friends_request),
                            ),
                    )
                    .service(
                        web::scope("/ws")
                            .app_data(Data::new(server.clone()))
                            .service(index),
                    ),
            )
    })
    .keep_alive(std::time::Duration::from_secs(75)) // timeout set because of errors from Nginx. 75 seconds might be long
    .bind(("0.0.0.0", 6000))?
    .run()
    .await
}
