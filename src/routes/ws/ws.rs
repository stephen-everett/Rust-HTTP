use actix_web::{web, web::{Data,Payload}, Error,HttpResponse, HttpRequest, get};
use actix::Addr;
use actix_web_actors::ws;
use std::time::{Duration, Instant};


use crate::websockets::{
    actors::{waiting_room::WaitingRoom, connected_user::ConnectedUser},
    etc::connection_pool::Server
};


#[get("connect")]
async fn index(req:HttpRequest, stream: web::Payload, server: Data<Addr<WaitingRoom>>) -> Result<HttpResponse, Error> {
    ws::start(ConnectedUser {
        hb: Instant::now(),
        lobby_id: String::from(""),
        user_id: String::from(""),
        username: String::from(""),
        room: String::from("main"),
        addr: Server::WaitingRoom(server.get_ref().clone())
    }, &req, stream)
}
