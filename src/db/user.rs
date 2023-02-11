use serde::Deserialize;
use serde::Serialize;

use crate::db::Id;

use mongodb::Client;
use mongodb::Cursor;
use mongodb::bson;
use mongodb::error::Error as DbError;

use crate::db::DATABASE;
use crate::db::Comment;

use super::Restaurant;


const USER_COLLECTION: &str = "users";

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct User {
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

impl User {
    pub async fn favorites(&self, client: &Client) -> Result<Cursor<Restaurant>, DbError> {
        client.database(DATABASE)
            .collection(USER_COLLECTION)
            .find(bson::doc! {
                "_id": {
                    "$in": &self.favorite_ids,
                },
            }, None)
            .await
    }

    pub async fn munch_list(&self, client: &Client) -> Result<Cursor<User>, DbError> {
        client.database(DATABASE)
            .collection(USER_COLLECTION)
            .find(bson::doc! {
                "_id": {
                    "$in": &self.munch_list_ids,
                },
            }, None)
            .await
    }

    pub async fn following_list(&self, client: &Client) -> Result<Cursor<User>, DbError> {
        client.database(DATABASE)
            .collection(USER_COLLECTION)
            .find(bson::doc! {
                "_id": {
                    "$in": &self.following_ids,
                },
            }, None)
            .await
    }

    pub async fn followers_list(&self, client: &Client) -> Result<Cursor<User>, DbError> {
        client.database(DATABASE)
            .collection(USER_COLLECTION)
            .find(bson::doc! {
                "_id": {
                    "$in": &self.follower_ids,
                },
            }, None)
            .await
    }

    pub async fn comments(&self, client: &Client) -> Result<Cursor<Comment>, DbError> {
        client.database(DATABASE)
            .collection(USER_COLLECTION)
            .find(bson::doc! {
                "_id": {
                    "$in": &self.comment_ids,
                },
            }, None)
            .await
    }

}