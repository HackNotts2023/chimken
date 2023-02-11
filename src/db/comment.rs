use serde::Deserialize;
use serde::Serialize;

use crate::impl_unique;
use crate::db::Id;
use crate::db::Votes;

const COMMENT_COLLECTION: &str = "comments";

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Comment {
    id: Id,
    message: String,
    votes: Votes,
    subcomment_ids: Vec<Id>,
}

impl_unique!(Comment => COMMENT_COLLECTION);