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
    #[serde(rename = "history_operations")]
    pub history_operations: Vec<HistoryOperation>,

    #[serde(rename = "status")]
    pub status: u8,
}

/// HistoryOperation
#[derive(Serialize, Deserialize)]
pub struct HistoryOperation {
    #[serde(rename = "history_operation_id")]
    pub history_operation_id: String,

    #[serde(rename = "init_args")]
    pub init_args: Vec<ArgPlaceholder>,

    #[serde(rename = "note")]
    pub note: String,

    #[serde(rename = "operations")]
    pub operations: Option<Vec<Operations>>,

    #[serde(rename = "time_stamp")]
    pub time_stamp: u64,
}

/// ArgPlaceholder
#[derive(Serialize, Deserialize)]
pub struct ArgPlaceholder {
    #[serde(rename = "arg_id")]
    pub arg_id: String,

    #[serde(rename = "dst_operation_id")]
    pub dst_operation_id: String,
}

/// Operations
#[derive(Serialize, Deserialize)]
pub struct Operations {
    #[serde(rename = "args")]
    pub args: Vec<ArgPlaceholder>,

    #[serde(rename = "method_name")]
    pub method_name: String,

    #[serde(rename = "module_name")]
    pub module_name: String,

    #[serde(rename = "operation_id")]
    pub operation_id: String,

    #[serde(rename = "output_image")]
    pub output_image: Vec<ImagePlaceholder>,

    #[serde(rename = "time_stamp")]
    pub time_stamp: i64,
}

/// ImagePlaceholder
#[derive(Serialize, Deserialize)]
pub struct ImagePlaceholder {
    #[serde(rename = "img_id")]
    pub img_id: String,

    #[serde(rename = "src_operation_id")]
    pub src_operation_id: String,
}