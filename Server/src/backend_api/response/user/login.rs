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


use std::ascii::escape_default;
use rocket::serde::{Deserialize, Serialize};
use crate::url;

#[derive(Serialize, Deserialize)]
pub struct Response {
    /// 权限，0为未登录状态，1为普通用户，2为管理员
    #[serde(rename = "authority")]
    pub authority: u8,

    /// 当为管理员权限时，返回manager的api_url，当为普通用户是为null
    #[serde(rename = "manager_url")]
    pub manager_url: Option<ManagerUrl>,

    /// 状态码，0为成功，
    /// 1为密码错误，
    /// 2为权限错误，
    #[serde(rename = "status")]
    pub status: u8,

    /// 时间戳
    #[serde(rename = "time_stamp")]
    pub time_stamp: u64,

    /// 令牌，由user_id, time_stamp, ip addr 加密所得到的哈希值
    #[serde(rename = "token")]
    pub token: String,
}

impl Response {
    pub fn new(authority: u8, status: u8, time_stamp: u64, token: String) -> Response {
        let manager_url = ManagerUrl::new(authority);

            return Response {
                authority: authority,
                manager_url: Option::from(manager_url),
                status: status,
                time_stamp: time_stamp,
                token: token
            }

    }
}

/// 当为管理员权限时，返回manager的api_url，当为普通用户是为null
#[derive(Serialize, Deserialize)]
pub struct ManagerUrl {
    #[serde(rename = "getAllOperatingMsg")]
    pub get_all_operating_msg: String,

    #[serde(rename = "getAllWebFiles")]
    pub get_all_web_files: String,

    #[serde(rename = "history_operations")]
    pub history_operations: HistoryOperations,

    #[serde(rename = "suggestion")]
    pub suggestion: Suggestion,

    #[serde(rename = "user")]
    pub user: User,
}

impl ManagerUrl {
    pub fn new(authority: u8) -> Option<ManagerUrl> {
        if authority == 2 {
            return Option::from(ManagerUrl {
                get_all_operating_msg: "/manager/get_all_operating_msg".to_string(),
                get_all_web_files: "/manager/get_all_webfiles".to_string(),
                history_operations: HistoryOperations::new(),
                suggestion: Suggestion::new(),
                user: User::new()
            })
        }else {
            return None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HistoryOperations {
    #[serde(rename = "getAllHistoryOperation")]
    pub get_all_history_operation: String,

    #[serde(rename = "getOnceOfHistoryOperation")]
    pub get_once_of_history_operation: String,
}

impl HistoryOperations{
    pub fn new()->HistoryOperations{
        return HistoryOperations{
            get_all_history_operation: url!("/manager/get_all_history_operation"),
            get_once_of_history_operation: url!("/manager/get_once_of_history_operation")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    #[serde(rename = "getAllSuggestions")]
    pub get_all_suggestions: String,

    #[serde(rename = "ignoreSuggestionByID")]
    pub ignore_suggestion_by_id: String,

    #[serde(rename = "submitResponseToSuggestionByID")]
    pub submit_response_to_suggestion_by_id: String,
}

impl Suggestion{
    pub fn new()->Suggestion{
        return Suggestion{
            get_all_suggestions: url!("/manager/get_all_suggestions"),
            ignore_suggestion_by_id: url!("/manager/ignore_suggestion_by_id"),
            submit_response_to_suggestion_by_id: url!("/manager/submit_response_to_suggestion_by_id")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "changeUserPwd")]
    pub change_user_pwd: String,

    #[serde(rename = "eraseUserMsg")]
    pub erase_user_msg: String,

    #[serde(rename = "getAllUserMsg")]
    pub get_all_user_msg: String,
}

impl User{
    pub fn new()->User{
        return User{
            change_user_pwd: url!("/manager/change_user_pwd"),
            erase_user_msg: url!("/manager/erase_user_msg"),
            get_all_user_msg: url!("/manager/get_all_user_msg"),
        }
    }
}