//! src/handlers.rs

use std::fmt::format;

use axum::{
  extract::{Extension, Json},
  response::{self, IntoResponse},
};
use hyper::http::StatusCode;
use sqlx::{
  postgres::{PgPool, PgRow},
  query_as,
};
use uuid::Uuid;

use crate::models;

use models::RecipeIn;

pub async fn index() -> response::Html<&'static str> {
  response::Html("<h1>Hello, friend!</h1>")
}

pub async fn get_recipes(
  Extension(pool): Extension<PgPool>,
) -> response::Json<models::RecipesInfo> {
  let recipe = query_as!(
    models::RecipesInfo,
    r#"
SELECT id, title
FROM recipes
ORDER BY id
limit 1
    "#
  )
  .fetch_one(&pool)
  .await
  .unwrap();

  response::Json(recipe)
}

pub async fn get_recipe(recipe: uuid::Uuid) -> response::Json<models::RecipeOut> {
  todo!()
}

pub async fn delete_recipe(recipe: uuid::Uuid) -> response::Json<String> {
  todo!()
}

pub async fn create_recipe(
  Extension(pool): Extension<PgPool>,
  Json(recipe): Json<models::RecipeIn>,
) -> impl IntoResponse {
  let recipe_id = Uuid::new_v4();

  sqlx::query!(
    "INSERT INTO recipes (id, title, content, published) VALUES ($1, $2, $3, $4)",
    recipe_id,
    recipe.title,
    recipe.content,
    recipe.published,
  )
  .execute(&pool)
  .await
  .expect("Failed to insert to DB");

  (
    StatusCode::CREATED,
    response::Json(format!("Created recipe with id: {}", recipe_id)),
  )
}

pub async fn update_recipe(
  recipe: models::Recipe,
) -> response::Json<models::RecipeOut> {
  todo!()
}
