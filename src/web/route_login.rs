use crate::{Result, Error};
use axum::Json;
use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]

struct Logindetails{
    username:String,
    pwd:String
}

async fn Login_api(payload:Json<Logindetails>)->Result<Json<Value>>{
  if payload.username == "solomon" && payload.pwd =="password"{}
  todo!()
}