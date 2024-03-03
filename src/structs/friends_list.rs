use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize,FromRow)]
struct FriendList{
    friend_list: Vec<User>,
    pending_friend_request: Vec<User>,
    accept_friend: bool,
    decline_friend: bool,

}