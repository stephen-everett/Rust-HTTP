
/*
    These are structures used for friends list and friends requests
 */

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize,FromRow)]
pub struct Friend{
    pub friend_id: String,
    pub username: String,
    pub status:String
}

#[derive(Deserialize)]
pub struct RequestId{
    pub user_id: String,
}

