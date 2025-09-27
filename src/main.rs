use axum::{extract::{Path, Query}, response::IntoResponse, routing::{get, get_service}, Router};
use tower_http::services::ServeDir;
use serde::Deserialize;
use std::net::SocketAddr;
pub use self::errors::{Error, Result};

#[tokio::main]

async fn main(){
  let routes = Router::new().merge(dynamic_routes()).fallback_service(static_routes());
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

  axum::serve(listener,routes.into_make_service()).await.unwrap();
}

mod errors;
mod web;

fn dynamic_routes()->Router{
  Router::new().route("/home", get(query_param))
               .route("/home/{id}", get(path_param))
}
#[derive(Debug, Deserialize)]

struct NameProfiler{
  name: Option<String>
}
async fn query_param(Query(payload):Query<NameProfiler>)-> impl IntoResponse{
  match payload.name{
    Some(n)=> format!("hello {n}, welcome to our platform"),
    None=> "hello user, welcome to our platform".to_string()
  }
}

async fn path_param(Path(user_id): Path<String>)-> impl IntoResponse{
   format!("the user id of the page is found in {}", user_id)
}

fn static_routes()-> Router{
   Router::new().nest_service("/path", get_service(ServeDir::new("/static")))
}