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
