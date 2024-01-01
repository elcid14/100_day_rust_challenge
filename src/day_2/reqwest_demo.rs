use core::fmt;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Number;

type CustomResult<T> = Result<T, CustomError>;

#[derive(Debug)]
pub enum CustomError{
    HttpRequestError(reqwest::Error)
}

#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
pub struct Post{
    id:Number,
    title:String,
    body:String,
    author:String,
    comments:Vec<Comment>,
    date:String
}

#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
pub struct Comment{
    id:Number,
    body:String,
    post_id:Number,

}

#[tokio::main]
pub async fn get_reqwest() -> CustomResult<()>{
    let body = reqwest::get("http://localhost:3000/posts")
    .await.unwrap()
    .text()
    .await.unwrap();

    let res: Vec<Post> = serde_json::from_str(&body).unwrap();
    println!("{:?}", res);
    Ok(())
}