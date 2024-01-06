use async_trait::async_trait;
use mongodb::{bson::Document, Collection};
use serde::{de::DeserializeOwned, Serialize};

/// This trait defines methods and behaviour of a model that's registered under the Mongorm struct. This trait will be implemented
/// on every model that is associated to the *MongormConnection* Struct.

#[async_trait]
pub trait MongoModel {
    async fn find_by_id<S>(_id: &S) -> Option<()>
    where
        S: Into<String>;

    async fn find(filter: Document) -> Option<()>;

    async fn get_all() -> Option<()>;
}

pub struct ModelOptions {
    pub time_stamps: bool,
    pub index: Vec<String>,
}

impl Default for ModelOptions {
    fn default() -> Self {
        Self {
            time_stamps: true,
            index: vec![],
        }
    }
}

pub struct Model<T>
where
    T: Serialize + DeserializeOwned,
{
    name: &'static str,
    collection: Collection<T>,
}

#[async_trait]
impl<T> MongoModel for Model<T>
where
    T: Serialize + DeserializeOwned,
{
    async fn find_by_id<S>(_id: &S) -> Option<()>
    where
        S: Into<String>,
    {
        None
    }

    async fn find(filter: Document) -> Option<()> {
        None
    }

    async fn get_all() -> Option<()> {
        None
    }
}
