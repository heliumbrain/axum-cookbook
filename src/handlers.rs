//! src/handlers.rs

use std::{env, ptr::hash};

use argonautica::{Hasher, Verifier};
use axum::{
  extract::{self, Extension, Json, UrlParams},
  response::{self, IntoResponse},
};
use hyper::http::StatusCode;
use serde::de::value::StringDeserializer;
use serde_json::{json, to_string};
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

  (StatusCode::OK, response::Json(json!({ "recipe_id": id })))
}

pub async fn create_user(
  Extension(pool): Extension<PgPool>,
  extract::Form(user): extract::Form<models::UserDbIn>,
) -> impl IntoResponse {
  let user_id = Uuid::new_v4();

  let mut hasher = Hasher::default();
  hasher.configure_hash_len(16);
  //Todo: Return a 201 and move the hashing to a background task (maybe with tokio::task)
  let password_hashed = hasher
    .with_password(&user.password)
    .with_secret_key(
      &env::var("SECRET_KEY").expect("SECRET_KEY not found in env variables"),
    )
    .hash()
    .unwrap();

  sqlx::query!(
    "INSERT INTO users (id, username, password_hashed) VALUES ($1, $2, $3)",
    user_id,
    user.username,
    password_hashed
  )
  .execute(&pool)
  .await
  .expect("Failed to insert to DB");

  (
    StatusCode::CREATED,
    response::Json(json!({ "user_id": user_id })),
  )
}

pub async fn login_user(
  Extension(pool): Extension<PgPool>,
  extract::Form(user): extract::Form<models::UserDbIn>,
) -> impl IntoResponse {
  let mut verifier = Verifier::default();

  let hash: String = sqlx::query!(
    "SELECT password_hashed FROM users where username = $1",
    user.username,
  )
  .fetch_one(&pool)
  .await
  .map(|r|{
    r.password_hashed
  })
  .unwrap();

  let is_valid_password = verifier
    .with_hash(hash)
    .with_password(user.password)
    .with_secret_key(&env::var("SECRET_KEY").expect("SECRET_KEY not found in env variables"))
    .verify()
    .expect("Password is not valid");

  match is_valid_password {
    true => {
      (
        StatusCode::OK,
        response::Json(json!({"Logged in": user.username})),
      )
    }

    false => {
      (
        StatusCode::UNAUTHORIZED,
        response::Json(json!({"Failed to login": user.username})),
      )
    }

  }

}
