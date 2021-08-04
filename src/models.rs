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
