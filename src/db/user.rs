use serde::Deserialize;
use serde::Serialize;

use crate::db::Id;

const USER_COLLECTION: &str = "users";

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: Id,
    name: String,
    bio: String,
    favorite_ids: Vec<Id>,
    munch_list_ids: Vec<Id>,
    following_ids: Vec<Id>,
    follower_ids: Vec<Id>,
    comment_ids: Vec<Id>,
}

impl_unique!(User => USER_COLLECTION);