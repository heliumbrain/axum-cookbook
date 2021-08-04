//! src/handlers.rs

use axum::{
  extract::{Extension, Json, UrlParams},
  response::{self, IntoResponse},
};
use hyper::http::StatusCode;
use serde_json::json;
use sqlx::{postgres::PgPool, query_as};
use uuid::Uuid;

use crate::models;

pub async fn get_recipes(
  Extension(pool): Extension<PgPool>,
) -> response::Json<Vec<models::RecipesOut>> {
  let recipe = query_as!(
    models::RecipesOut,
    r#"
    SELECT id, title
    FROM recipes
    ORDER BY title
    "#
  )
  .fetch_all(&pool)
  .await
  .unwrap();

  response::Json(recipe)
}

pub async fn get_recipe(
  Extension(pool): Extension<PgPool>,
  UrlParams((id,)): UrlParams<(Uuid,)>,
) -> response::Json<models::RecipeOut> {
  let recipe = query_as!(
    models::RecipeOut,
    r#"
    SELECT id, title, content
    FROM recipes
    WHERE id = $1
    "#,
    id
  )
  .fetch_one(&pool)
  .await
  .unwrap();

  response::Json(recipe)
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
    response::Json(json!({ "recipe_id": recipe_id })),
  )
}

pub async fn update_recipe(
  Extension(pool): Extension<PgPool>,
  Json(recipe): Json<models::RecipeUpdate>,
) -> impl IntoResponse {
  todo!()
}


pub async fn delete_recipe(
  Extension(pool): Extension<PgPool>,
  UrlParams((id,)): UrlParams<(Uuid,)>,
) -> impl IntoResponse {
  sqlx::query!(
    r#"
    DELETE FROM recipes
    WHERE id = $1
    "#,
    id
  )
  .execute(&pool)
  .await
  .expect("Failed to delete row from DB");

  (
    StatusCode::OK,
    response::Json(json!({ "recipe_id": id })),
  )

}
