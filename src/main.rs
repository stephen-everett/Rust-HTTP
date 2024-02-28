use actix_web::web::get;
/*
    Third Party Module
 */
// actix
use actix_web::{web, web::Data, App, HttpServer, Error, HttpRequest, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;

// actix websockets
use actix_web_actors::ws;
use actix::{Actor, StreamHandler, Addr};
use actix_web::get;

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
    app::{delete_user::delete_user, get_user_info::user_info, search::search_user,
          update_user::{update_first_name,update_last_name,update_email,update_password,update_pin,update_username,update_phone_number},
          friends::{send_friend_request, accept_friend_request, get_accepted_friends, get_outgoing_friends, get_incoming_friends, deny_friends_request}
        },
    auth::{login::basic_auth, register::create_user},
    debug::{get_all_users, test_auth, test_connection},
    ws::ws::index
};


// Validate JWT (Authentication)
//mod middleware;
use hello_rocket::middleware::validator::validator;

//mod experimental;
//use experimental::chat::actors::{connected_user::{ConnectedUser, Server}, waiting_room::WaitingRoom};

//mod websockets;
use hello_rocket::websockets::actors::{connected_user::ConnectedUser, waiting_room::WaitingRoom};
use hello_rocket::websockets::etc::connection_pool::Server;


/*
    Temport inline echo server
 */
/* 
#[get("echo")]
async fn index(req:HttpRequest, stream: web::Payload, server: Data<Addr<WaitingRoom>>) -> Result<HttpResponse, Error> {
    ws::start(ConnectedUser {
        user_id: String::from(""),
        username: String::from(""),
        room: String::from("main"),
        addr: Server::WaitingRoom(server.get_ref().clone())
    }, &req, stream)
}
*/



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
    

    let server = WaitingRoom::new(Data::new(AppState { db: pool.clone() })).start();
    HttpServer::new(move || {
        // bearer middleware used to verify JWT token on protected routes.
        let bearer_middleware = HttpAuthentication::bearer(validator);

        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
                .service(
                    web::scope("/api")
                    .service(create_user)
                    .service(basic_auth)
                    .service(
                        web::scope("/debug")
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
                        .service(user_info)
                        .service(
                            web::scope("/update")
                            .service(update_first_name)
                            .service(update_last_name)
                            .service(update_email)
                            .service(update_password)
                            .service(update_username)
                            .service(update_pin)
                            .service(update_phone_number)
                        )
                        .service(
                            web::scope("/friends")
                            .service(send_friend_request)
                            .service(accept_friend_request)
                            .service(get_accepted_friends)
                            .service(get_incoming_friends)
                            .service(get_outgoing_friends)
                            .service(deny_friends_request)
                        )
                    )
                    .service(
                        web::scope("/ws")
                        .app_data(Data::new(server.clone()))
                        .service(index)
                        
                    )
                )
    })
    .keep_alive(std::time::Duration::from_secs(75)) // timeout set because of errors from Nginx. 75 seconds might be long
    .bind(("0.0.0.0", 6000))?
    .run()
    .await
}