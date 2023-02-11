use mongodb::Client;
use mongodb::Cursor;
use mongodb::bson;
use mongodb::error::Error as DbError;

use serde::Deserialize;
use serde::Serialize;

use crate::db::DATABASE;
use crate::db::Comment;
use crate::db::Id;
use crate::db::Votes;

const RESTAURANT_COLLECTION: &str = "restaurants";

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Restaurant {
    #[serde(rename = "_id")]
    id: Id,
    name: String,
    votes: Votes,
    comment_ids: Vec<Id>,
}

impl_unique!(Restaurant => RESTAURANT_COLLECTION);

impl Restaurant {
    pub async fn comments(&self, client: &Client) -> Result<Cursor<Comment>, DbError> {
        client.database(DATABASE)
            .collection(RESTAURANT_COLLECTION)
            .find(bson::doc! {
                "_id": {
                    "$in": &self.comment_ids,
                },
            }, None)
            .await
    }
}