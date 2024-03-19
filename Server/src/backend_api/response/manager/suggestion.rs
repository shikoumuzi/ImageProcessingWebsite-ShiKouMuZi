// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

// extern crate serde_derive;

use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    /// 状态码
    #[serde(rename = "status")]
    pub status: u8,

    /// 建立列表
    #[serde(rename = "suggestions")]
    pub suggestions: Vec<Suggestion>,
}

impl Response {
    pub fn new(status: u8, suggestions: Vec<Suggestion>) -> Self {
        Self { status, suggestions }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "suggestion_id")]
    pub suggestion_id: String,

    #[serde(rename = "time_stamp")]
    pub time_stamp: u64,

    #[serde(rename = "username")]
    pub username: String,
}

impl Suggestion {
    pub fn new(content: String, suggestion_id: String, time_stamp: u64, username: String) -> Self {
        Self { content, suggestion_id, time_stamp, username }
    }
}