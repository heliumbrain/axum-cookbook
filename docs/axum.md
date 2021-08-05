# Axum for dummies

#### Getting started

## About Axum

`axum` is a new web application framework that (according to their repo) focuses on ergonomics and modularity.

## Why Axum?

## Create your project

In your terminal of choice, go to your favorite code-directory and run:

`cargo init axum-dummy && cd axum-dummy`

Before getting started with our Axum-app, we will need to add some of the basic dependencies needed to our `Cargo.toml` file, like so:

```other
...

[dependencies]
axum = "0.1"
tokio = { version = "1.9.0", features = ["full"] }
hyper = { version = "0.14", features = ["full"] }
```

[**Tokio**](https://github.com/tokio-rs/tokio) - The async runtime that powers it all. Tokio is fast and reliable.

[**Hyper**](https://github.com/hyperium/hyper) -  Async HTTP implementation that acts as the HTTP server for Axum

> **NOTE!** t's possible to use Axum with another combination of runtime and HTTP server. Tokio + Hyper just happens to suit well and is what is used by the authors of Axum. Therefore, this series will stick to using that, for now :)

To check that everything is working as intended, let's create a simple Hello World app, based on the [example](https://github.com/tokio-rs/axum/blob/main/examples/hello_world.rs) in the [Axum repo](https://github.com/tokio-rs/axum).

Add the following to `src/main.rs:`

```rust
use axum::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> response::Html<&'static str> {
    response::Html("<h1>Hello, friend!</h1>")
}
```

> **NOTE!** I have purposely left out the `tracing` parts that are being used in the example from the repo. We will get to this later in this series. This is the most barebone way of getting a Axum-app up and running.

After running the project with `cargo run` you should be able to visit [`http://127.0.0.1:3000`](http://127.0.0.1:3000) and be greeted by Axum.

## Learn more and get involved

[GitHub - tokio-rs/axum: Ergonomic and modular web framework built with Tokio, Tower, and Hyper](https://github.com/tokio-rs/axum)

[axum - Rust](https://docs.rs/axum/0.1.2/axum/index.html)

----

#### CRUD Routes

```rust
//! src/main.rs

...

mod handlers;

...

let app = route(
    "/recipes",
    get(handlers::get_recipes)
      .post(handlers::create_recipe)
      .delete(handlers::delete_recipe)
      .patch(handlers::update_recipe),
  );
```

Set up your handlers in `src/handlers.rs`

```rust
//! src/handlers.rs

use axum::response;

use crate::models;

pub async fn handler() -> response::Html<&'static str> {
  response::Html("<h1>Hello, friend!</h1>")
}

pub async fn get_recipes() -> response::Json<Vec<models::RecipesOut>> {
  todo!()
}

pub async fn get_recipe(recipe: uuid::Uuid) -> response::Json<models::RecipeOut> {
  todo!()
}

pub async fn delete_recipe(recipe: uuid::Uuid) -> response::Json<String> {
  todo!()
}

pub async fn create_recipe(
  recipe: models::RecipeIn,
) -> response::Json<models::RecipeOut> {
  todo!()
}

pub async fn update_recipe(
  recipe: models::Recipe,
) -> response::Json<models::RecipeOut> {
  todo!()
}
```

----

#### Structs and Types

New dependenices in `cargo.toml`

```other
uuid = { version = "0.8", features = ["serde", "v4"] }
serde = { version = "1.0.127", features = ["derive"] }
serde_json = "1.0.66"
```

Create the following structs in `src/models.rs`

```rust
//! src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
  id: uuid::Uuid,
  title: String,
  content: String,
  author: User,
  published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeOut {
  id: uuid::Uuid,
  title: String,
  content: String,
  author: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeIn {
  title: String,
  content: String,
  author: User,
  published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipesOut {
  id: uuid::Uuid,
  title: String,
  author: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
  id: uuid::Uuid,
  username: String,
  password: String,
}

#[derive(Serialize, Debug)]
struct UserDbIn {
  username: String,
  hashed_password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserOut {
  id: uuid::Uuid,
  username: String,
}
```

----

#### Connect to a database using sqlx

New dependencies in `Cargo.toml`

```other
sqlx = { version = "0.5", features = [ "runtime-tokio-rustls", "postgres", "macros", "uuid", "time" ] }
```

Install the sqlx-cli

`cargo install sqlx-cli --no-default-features --features postgres`

Create a `.env` file in your project root, specifying the Postgres DB sqlx should use

----

#### Parsing Form data

----

#### Parsing and returning JSON data

----

#### Insert, Select, Alter and Drop your data

----

#### Managing Users

----

#### Authorization and Authentication

----

#### Creating, reading and managing cookies

