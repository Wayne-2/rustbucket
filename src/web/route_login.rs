use crate::{Result, Error};
use axum::{routing::post, Json, Router};
use serde::{Deserialize};
use serde_json::{json, Value};

pub fn api_route()->Router{
  Router::new().route("api/api_login", post(login_api))
}

#[derive(Debug, Deserialize)]

struct Logindetails{
    username:String,
    pwd:String
}

async fn login_api(payload:Json<Logindetails>)->Result<Json<Value>>{
  if payload.username != "solomon" || payload.pwd !="password"{
    return Err(Error::Loginfailed);
  }

  let body = Json(json!({
    "result":{
      "success":true
    }
  }));
  Ok(body)
}