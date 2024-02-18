use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

// Structure for connecting to postgres database
// Author: Stephen Everett
pub struct AppState {
    pub db: Pool<Postgres>
}

// Structure that contains data embedded in JWT token used for authentication
// More data can be embedded as necessary
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub user_id:String,
}