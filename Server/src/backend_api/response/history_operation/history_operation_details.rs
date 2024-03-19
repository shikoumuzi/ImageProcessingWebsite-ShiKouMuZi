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
    /// 操作详情，所有操作列表
    #[serde(rename = "operation_details")]
    pub operation_details: Vec<Operations>,

    /// 状态码，0为成功，1为失败
    #[serde(rename = "status")]
    pub status: u8,
}

impl Response {
    pub fn new(operation_details: Vec<Operations>, status: u8) -> Self {
        Self { operation_details, status }
    }
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
    pub time_stamp: u64,
}

impl Operations {
    pub fn new(args: Vec<ArgPlaceholder>, method_name: String, module_name: String, operation_id: String, output_image: Vec<ImagePlaceholder>, time_stamp: u64) -> Self {
        Self { args, method_name, module_name, operation_id, output_image, time_stamp }
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