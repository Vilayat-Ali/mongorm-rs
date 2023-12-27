//! # MongORM
//!
//! MongORM-RS is a fast, simple, and powerful Object-Document Mapper (ODM) for MongoDB in Rust. It provides an ORM-like API to interact with MongoDB databases seamlessly.

pub mod error;

pub use error::Error;

use mongodb::{options::ClientOptions, Client};

pub async fn establish_connection(conn_str: &str) -> crate::Error {
    let mut client_options = ClientOptions::parse(conn_str).await.unwrap();
    Error::ConnectionFailed("sad".to_owned())
}
