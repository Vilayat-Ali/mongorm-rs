[![Logo](<link-to-your-logo-image> "Logo")](<link-to-your-github-repo "Logo">)

# MongORM-RS

#### MongoDB ODM for Rustlang

MongORM-RS is a fast, simple, and powerful Object-Document Mapper (ODM) for MongoDB in Rust. It provides an ORM-like API to interact with MongoDB databases seamlessly.

[![crates.io](https://img.shields.io/crates/v/mongorm-rs "crates.io")](https://crates.io/crates/mongorm-rs "crates.io")
[![crates.io](https://img.shields.io/static/v1?label=rustc&message=v1.57%2B&color=red "crates.io")](https://www.rust-lang.org/ "crates.io")
![crates.io](https://img.shields.io/crates/l/mongorm-rs)
![crates.io](https://img.shields.io/crates/d/mongorm-rs)

## Features

- ORM-like API for MongoDB in Rust
- Fast and efficient interactions with MongoDB databases
- Simplified alternative for simulating relationships (Many-to-One and Many-to-Many)
- Supports selective fields with dynamic addition of elements
- Tested on MongoDB versions 4.2, 4.4, 5.0, 6.0

## Installation

Run this command at root of your project

```bash
cargo add mongorm
```

Or, add this to your `Cargo.toml`

```toml
mongorm = "0.1.0"
```                   

## Basic Usage

```rust
use mongorm::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    task_id: String,
    title: String,
    is_completed: bool
}

#[tokio::main]
async fn main() -> Result<(), mongorm::errors::Error> {
    // establish connection to db
    establish_connection();

    // create a model
    let todo_model = model!(Todo);

    // perform CRUD
    
    // CREATE
    let new_todo = todo_model.create<Todo>(Todo {
        task_id: 1,
        title: "Learn Rust".to_owned(),
        is_completed: false
    });

    // READ
    let todos = todo_model.find(None); 

    // UPDATE
    let _ = todo_model.findByIdAndUpdate<Todo>("...", Todo {
        task_id: 1,
        title: "Learn MONGODB".to_owned(),
        is_completed: false
    });

    // DELETE
    let - = todo_model.findByIdAndDelete<Todo>("...");
}

```

## Changelog

[View the change history.](<link-to-your-github-repo/blob/main/CHANGELOG.md "View the change history.")

## License

#### This project is licensed under the [MIT](<link-to-your-github-repo/LICENSE-MIT "MIT") and [Apache Version 2.0](<link-to-your-github-repo/LICENSE-APACHE "Apache Version 2.0")