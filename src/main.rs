use axum::{Router, routing::get, extract::Query, response::IntoResponse};
use std::net::SocketAddr;
use serde::Deserialize;

#[tokio::main]

async fn main(){
  let route = Router::new().route("/", get(handler_func));
  let addr = SocketAddr::from(([127, 0, 0 , 1], 3000));
  println!("the server is running at port: {}", addr);

  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, route.into_make_service()).await.unwrap();
}
#[derive(Debug, Deserialize)]
struct Userinfo{
  username: Option<String>,
}

async fn handler_func(Query(params):Query<Userinfo>)-> impl IntoResponse{
    match params.username{
      Some(n)=> format!("hello, {n}, welcome!"),
      None => "hello random user".to_string(),
    }
}
