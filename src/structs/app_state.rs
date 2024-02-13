use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub user_id:String,
}