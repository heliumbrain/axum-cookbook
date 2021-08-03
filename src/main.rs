use axum::prelude::*;
use std::net::SocketAddr;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = route(
        "/recipes",
        get(handlers::get_recipes)
            .post(handlers::create_recipe)
            .delete(handlers::delete_recipe)
            .patch(handlers::update_recipe),
    );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
