use actix_web::{
    post,
    web::{Data, Json},
    Responder, HttpResponse, web
};
use crate::structs::{
    app_state::AppState,
    menu_item::MenuItem,
    receipt_item::ReceiptItem,
    lobby::Lobby
};
use rand::Rng;

// retrieve lobby number, insert menu items into their respective table, and then create the realationship between
// menu item and lobby by inserting items into receipt_item table
#[post("/post_receipt")]
pub async fn post_receipt(state:Data<AppState>, body: web::Json<Vec<MenuItem>>) -> impl Responder {
    let lobby_number = futures::executor::block_on(create_new_lobby(state.clone()));
    for menu_item in body.into_inner() {
        {
            match sqlx::query!("INSERT INTO menu_item(SKU, name, quantity) VALUES($1,$2,$3)",menu_item.sku, menu_item.name, menu_item.quantity)
                .execute(&state.db)
                .await 
                {
                    Ok(_) => (),
                    Err(_) => print!("Unable to insert menu item into table"),
                };

           
            match sqlx::query!("INSERT INTO receipt_item(lobby_id, SKU) VALUES($1,$2)", lobby_number, menu_item.sku)
                .execute(&state.db)
                .await 
                {
                    Ok(_) => (),
                    Err(_) => print!("Unable to insert receipt item into table"),
                };
        }
    }
    HttpResponse::Ok().json(lobby_number)
}

// return all menu items associated with a given lobby number
#[post("/join_lobby")]
pub async fn join_lobby(state: Data<AppState>, body: Json<Lobby>) -> impl Responder {
    let query_string = format!("SELECT lobby_id, SKU, name, quantity FROM receipt_item JOIN lobby USING(lobby_id) JOIN menu_item USING(sku) WHERE lobby_id = {}", body.lobby_id);
    match sqlx::query_as!(
        ReceiptItem,
        "SELECT lobby_id, sku, name, quantity FROM receipt_item JOIN lobby USING(lobby_id) JOIN menu_item USING(sku) WHERE lobby_id = $1",
        body.lobby_id
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json( query_string),
    }  
}

// create a new lobby and return lobby number
pub async fn create_new_lobby(state:Data<AppState>) -> i32 {
    let mut rng = rand::thread_rng();
    let lobby_number = rng.gen::<i32>();
    match sqlx::query!("INSERT INTO lobby VALUES($1)",lobby_number)
    .execute(&state.db)
    .await
    {
        Ok(_) => lobby_number,
        Err(_) => 0
    };
    lobby_number
}