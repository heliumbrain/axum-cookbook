//! src/models.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
  pub id: uuid::Uuid,
  pub title: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeIn {
  pub title: String,
  pub content: String,
  pub published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipesOut {
  pub id: Uuid,
  pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeUpdate {
  pub title: String,
  pub content: String,
  pub published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  id: uuid::Uuid,
  pub username: String,
  password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDbIn {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserOut {
  pub id: uuid::Uuid,
  pub username: String,
}
