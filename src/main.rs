use axum::{
  extract::{Path, Query}, response::IntoResponse, routing::{get, get_service}, Router
};
use std::net::SocketAddr;
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]

async fn main(){
  let app = Router::new()
  .merge(router_handlers())
  .fallback_service(static_route());
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

  axum::serve(listener, app.into_make_service()).await.unwrap();
}

fn router_handlers() -> Router{
  Router::new()
  .route("/home", get(user_home_profiler))
  .route("/home/{id}", get(user_id_resolver))
}
#[derive(Debug, Deserialize)]

struct Newuserlaucher{
  name: Option<String>
}
async fn user_home_profiler(Query(payloader):Query<Newuserlaucher>)-> impl IntoResponse{
   match payloader.name {
       Some(n)=> format!("hello, distinguished {n}"),
       None=>"hello random user, please sign in".to_string()
   }
}

async fn user_id_resolver(Path(user_id): Path<String>)-> impl IntoResponse{
  format!("hello user{}", user_id)
}

fn static_route()->Router{
  Router::new().nest_service("/", get_service(ServeDir::new("./assets")))
}

