mod model;
mod database;
mod endpoints;

use axum::{
    routing::{get, post},
    Router
};

use endpoints::{root::root, registration::registration};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/registration", post(registration));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}