use serde::Serialize;
use sqlx::FromRow;
use super::user::User;

#[derive(Serialize,FromRow)]
struct FriendList{
    friend_list: Vec<User>,
    pending_friend_request: Vec<User>,
    status: String

}