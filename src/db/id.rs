use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Deref;
use std::ops::DerefMut;

use mongodb::bson::Bson;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Id(i64);

impl Display for Id {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{}", self.0)
    }
}

impl Deref for Id {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Id> for Bson {
    fn from(id: Id) -> Self {
        Self::Int64(id.0)
    }
}

pub trait Unique {
    fn id(&self) -> Id;
}

#[macro_export]
macro_rules! impl_unique {
    ($($t: ty => $coll: expr),+$(,)?) => {
        $(
            impl $crate::db::id::Unique for $t {
                fn id(&self) -> $crate::db::id::Id {
                    self.id
                }
            }

            impl PartialEq for $t {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }

            impl $t {
                pub async fn having_id(id: $crate::db::id::Id, client: &::mongodb::Client) -> Result<Option<Self>, ::mongodb::error::Error> {
                    client.database($crate::db::DATABASE)
                        .collection($coll)
                        .find_one(::mongodb::bson::doc! {
                            "id": id,
                        }, None)
                        .await
                }
            }
        
        )+
    };
}