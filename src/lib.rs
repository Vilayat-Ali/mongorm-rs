//! # MongORM
//!
//! MongORM-RS is a fast, simple, and powerful Object-Document Mapper (ORM) for MongoDB in Rust. It provides an ORM-like API
//! to interact with MongoDB databases seamlessly. MongODM is designed tp blend-in easily with existing Rust-based web tooling
//! and frameworks like - Actix, Rocket, Axum, Hyper, etc.
//!
//! ## Features
//!
//! 1. Easily blends-in with your application
//! 2. Support for concurrent requests
//! 3. Supports Relationships
//!
//! ## Getting Started
//! ```no_run
//! use mongorm::prelude::*;
//!
//! #[derive(Model)]
//! pub struct Book {
//!     #[field(unique="false", required="true")]
//!     pub name: String,
//!     #[field(unique="true", required="true")]
//!     pub ISBN: String,
//!     #[field(unique="true", required="false")]
//!     pub author: Author,
//! }
//! #[derive(Model)]
//! pub struct Author {
//!     #[field(unique="false", required="true")]
//!     pub name: String,
//! }
//!
//! async fn add_book(conn: &Connection, book: &Book, author: Option<Author>) -> Result<CreationResult> {
//!     let result = conn.models.BookModel.create(book, author).await?;
//!     Ok(result)
//! }
//!
//! [tokio::main]
//! async fn main() -> Result<()> {
//!     // load connection string from env or hard code
//!     let conn_str: &'static str = "mongodb://0.0.0.0:27017/mongorm-test";
//!     let options = Mongorm::ConnOptions::default();
//!     let conn = Mongorm::establish_conn(conn_str, options).await?;
//!
//!     // register your models
//!     conn.add_models(vec![
//!     // books
//!     Book,
//!     // authors
//!     Author
//!     ]);
//!
//!     // use
//!     add_book(conn, .., ..).await.unwrap();
//!
//!     Ok(())
//! }
//! ```

pub mod model;
pub mod prelude;
