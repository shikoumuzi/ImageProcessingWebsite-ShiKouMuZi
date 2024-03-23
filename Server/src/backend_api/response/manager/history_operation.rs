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

impl Response {
    pub fn new(history_operations: Vec<HistoryOperation>, status: u8) -> Self {
        Self { history_operations, status }
    }
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
    pub operations: Option<Vec<Operation>>,

    #[serde(rename = "time_stamp")]
    pub time_stamp: u64,
}

impl HistoryOperation {
    pub fn new(history_operation_id: String, init_args: Vec<ArgPlaceholder>, note: String, operations: Option<Vec<Operations>>, time_stamp: u64) -> Self {
        Self { history_operation_id, init_args, note, operations, time_stamp }
    }
}

/// ArgPlaceholder
#[derive(Serialize, Deserialize)]
pub struct ArgPlaceholder {
    #[serde(rename = "arg_id")]
    pub arg_id: String,

    #[serde(rename = "dst_operation_id")]
    pub dst_operation_id: String,
}

impl ArgPlaceholder {
    pub fn new(arg_id: String, dst_operation_id: String) -> Self {
        Self { arg_id, dst_operation_id }
    }
}

/// Operations
#[derive(Serialize, Deserialize)]
pub struct Operation {
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

impl Operation {
    pub fn new(args: Vec<ArgPlaceholder>, method_name: String, module_name: String, operation_id: String, output_image: Vec<ImagePlaceholder>, time_stamp: i64) -> Self {
        Self { args, method_name, module_name, operation_id, output_image, time_stamp }
    }
}

/// ImagePlaceholder
#[derive(Serialize, Deserialize)]
pub struct ImagePlaceholder {
    #[serde(rename = "img_id")]
    pub img_id: String,

    #[serde(rename = "src_operation_id")]
    pub src_operation_id: String,
}

impl ImagePlaceholder {
    pub fn new(img_id: String, src_operation_id: String) -> Self {
        Self { img_id, src_operation_id }
    }
}