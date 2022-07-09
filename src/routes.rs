use axum::{
  routing::{get, post},
  http::StatusCode,
  response::IntoResponse,
  Json, Router, Extension,
};
use serde::{Deserialize, Serialize};

use crate::{models::*, db::Database};


// basic handler that responds with a static string
pub async fn root() -> &'static str {
  "Hello, World!"
}

pub async fn create_user(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  
  Json(payload): Json<CreateUser>,
  Extension(db): Extension<Database>,
) -> impl IntoResponse {
  db.create_user(payload).await;

  // insert your application logic here
  // let user = User {
  //   id: 1337,
  //   username: payload.username,
  //   email: "".to_owned(),
  //   hash: "".to_owned(),
  //   salt: "".to_owned(),
  // };

  // this will be converted into a JSON response
  // with a status code of `201 Created`
  StatusCode::CREATED
}

pub async fn auth_user(
  Json(payload): Json<AuthUser>,
  Extension(db): Extension<Database>,
) -> impl IntoResponse {
  let auth_success = db.auth_user(payload).await;
  if auth_success {
    StatusCode::OK
  } else {
    StatusCode::UNAUTHORIZED
  }
}

#[derive(Serialize)]
struct Sum {
  pub sum: i32,
}

pub async fn get_meters_sum(
  Json(payload): Json<IdUser>,
  Extension(db): Extension<Database>,
) -> impl IntoResponse {
  let sum = Sum {
    sum: db.get_meters_sum(&payload.username).await,
  };

  (StatusCode::OK, Json(sum))
}