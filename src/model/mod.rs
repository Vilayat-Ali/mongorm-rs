use async_trait::async_trait;
use mongodb::{bson::Document, Collection};
use serde::{de::DeserializeOwned, Serialize};

/// This trait defines methods and behaviour of a model that's registered under the Mongorm struct. This trait will be implemented
/// on every model that is associated to the *MongormConnection* Struct.

#[async_trait]
pub trait MongoModel {
    async fn find_by_id<S>(&self, _id: &S) -> Option<()>
    where
        S: Into<String>;

    async fn find(&self, filter: Document) -> Option<()>;

    async fn get_paginated_data(
        &self,
        page: u32,
        page_limit: u32,
        search_value: Option<String>,
        newest_first: bool,
    ) -> Option<()>;

    async fn find_one(&self, filter: Document) -> Option<()>;

    async fn update_one(&self, filter: Document, update: Document) -> Option<()>;

    async fn aggregate(&self, pipelines: Vec<Document>) -> Option<()>;
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
