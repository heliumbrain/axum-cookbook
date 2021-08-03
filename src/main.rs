use axum::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> response::Html<&'static str> {
    response::Html("<h1>Hello, friend!</h1>")
}
