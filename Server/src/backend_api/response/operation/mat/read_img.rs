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

use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "mat_index")]
    pub mat_index: i32,

    #[serde(rename = "status")]
    pub status: u8,
}

impl Response {
    pub fn new(status: u8, mat_index: i32) -> Self {
        Self { mat_index, status }
    }
}