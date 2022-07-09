mod routes;
mod models;
mod db;

use axum::{
    routing::{get, post}, Router, Extension,
};

use std::net::SocketAddr;

use crate::{routes::*, db::init_db};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let db = init_db().await;
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/create_user", post(create_user)).layer(Extension(db.clone()))
        .route("/auth_user", post(auth_user)).layer(Extension(db.clone()))
        .route("/get_meters_sum", post(get_meters_sum)).layer(Extension(db.clone()));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}