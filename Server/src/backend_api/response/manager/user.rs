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
    #[serde(rename = "status")]
    pub status: u8,

    #[serde(rename = "user_msg")]
    pub user_msg: Vec<UserMsg>,
}

#[derive(Serialize, Deserialize)]
pub struct UserMsg {
    #[serde(rename = "authority")]
    pub authority: i64,

    #[serde(rename = "history_operation_count")]
    pub history_operation_count: u64,

    #[serde(rename = "result_image_count")]
    pub result_image_count: u64,

    #[serde(rename = "time_stamp")]
    pub time_stamp: u64,

    #[serde(rename = "username")]
    pub username: String,
}