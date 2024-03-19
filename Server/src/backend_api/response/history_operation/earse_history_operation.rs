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
    /// 状态码，0为成功，1为失败
    #[serde(rename = "status")]
    pub status: u8,
}

impl Response {
    pub fn new(status: u8) -> Self {
        Self { status }
    }
}