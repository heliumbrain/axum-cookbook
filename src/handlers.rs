use axum::response;
use hyper::server::conn::Http;

pub async fn handler() -> response::Html<&'static str> {
    response::Html("<h1>Hello, friend!</h1>")
}

pub async fn get_recipes() -> response::Json<models::RecipesOut> {
    todo!()
}

pub async fn get_recipe() -> response::Json<models::RecipeOut> {
    todo!()
}

pub async fn create_recipe(recipe: models::RecipeIn) -> response::Json<models::RecipeOut> {
    todo!()
}

pub async fn update_recipe(recipe: models::Recipe) -> response::Json<models::RecipeOut> {
    todo!()
}
