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
  id: uuid::Uuid,
  title: String,
  content: String,
  author: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeIn {
  pub title: String,
  pub content: String,
  //pub author: User,
  pub published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipesOut {
  id: Uuid,
  title: String,
  author: UserOut,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipesInfo {
  pub id: Uuid,
  pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  id: uuid::Uuid,
  username: String,
  password: String,
}

#[derive(Serialize, Debug)]
pub struct UserDbIn {
  username: String,
  hashed_password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserOut {
  id: uuid::Uuid,
  username: String,
}

