use mongodb::Client;
use mongodb::Cursor;
use mongodb::bson;
use mongodb::error::Error as DbError;

use serde::Deserialize;
use serde::Serialize;

use crate::impl_unique;
use crate::db::DATABASE;
use crate::db::Id;
use crate::db::Votes;

const COMMENT_COLLECTION: &str = "comments";

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "_id")]
    id: Id,
    message: String,
    votes: Votes,
    subcomment_ids: Vec<Id>,
}

impl_unique!(Comment => COMMENT_COLLECTION);

impl Comment {
    pub async fn subcomments(&self, client: &Client) -> Result<Cursor<Self>, DbError> {
        client.database(DATABASE)
            .collection(COMMENT_COLLECTION)
            .find(bson::doc! {
                "_id": {
                    "$in": &self.subcomment_ids,
                },
            }, None)
            .await
    }
}