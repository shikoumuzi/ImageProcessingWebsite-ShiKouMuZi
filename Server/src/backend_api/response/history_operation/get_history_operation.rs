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
    /// 历史操作列表
    #[serde(rename = "history_operations")]
    pub history_operations: Vec<HistoryOperation>,

    /// 状态码，0为成功，1为失败
    #[serde(rename = "status")]
    pub status: u8,
}

impl Response {
    pub fn new(history_operations: Vec<HistoryOperation>, status: u8) -> Self {
        Self { history_operations, status }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HistoryOperation {
    /// 历史操作id
    #[serde(rename = "history_operation_id")]
    pub history_operation_id: String,

    /// 备注
    #[serde(rename = "note")]
    pub note: String,

    /// 创建时间戳
    #[serde(rename = "time_stamp")]
    pub time_stamp: u64,
}

impl HistoryOperation {
    pub fn new(history_operation_id: String, note: String, time_stamp: u64) -> Self {
        Self { history_operation_id, note, time_stamp }
    }
}