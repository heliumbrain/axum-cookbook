//! src/main.rs

use axum::{prelude::*, AddExtensionLayer};
use dotenv;
use handlers::{get_recipe, create_recipe, get_recipes, delete_recipe};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
mod handlers;
mod models;

#[tokio::main]
async fn main() {
  dotenv::dotenv().expect("Failed to read .env file");

  //Create a connection pool for Postgres DB
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(
      &env::var("DATABASE_URL").expect("DATABASE_URL not found in env variables"),
    )
    .await
    .unwrap();

  // build our application with a route
  let app = route(
    "/recipes",
    get(get_recipes).post(create_recipe),
  )
  .route("/recipes/:id", get(get_recipe).delete(delete_recipe))
  .layer(AddExtensionLayer::new(pool));

  // run it
  let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
  hyper::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
