use std::sync::Mutex;
use std::vec;
use std::option::Option;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
// use serde::de::Unexpected::Option;
use crate::typings::user::user::UserGroup;
use super::response::manager::common::Response as CommonResponse;
use super::response::manager::suggestion::Response as SuggestionResponse;
use super::response::manager::suggestion::Suggestion;
use super::response::manager::user::{Response as UserResponse, UserMsg};
use super::response::manager::history_operation::{HistoryOperation, Response as HistoryOperationResponse};

// ================= suggestion ==============================

#[post("/image_processing_website_api/manager/get_all_suggestions?<token>&<now_len>")]
fn getAllSuggestions(token: String, now_len: u16) -> Json<SuggestionResponse>{
    let suggestion_response = SuggestionResponse::new(0, vec![Suggestion::new("".to_string(), "".to_string(), 0, "".to_string())]);
    Json(suggestion_response)
}

#[post("/image_processing_website_api/manager/submit_response_to_suggestion_by_id?<token>&<suggestion_id>&<response>")]
fn submitResponseToSuggestionById(users: &State<Mutex<UserGroup>>, token: String, suggestion_id: String, response: String) -> Json<CommonResponse>{
    let common_response = CommonResponse::new(0);
    Json(common_response)
}

#[post("/image_processing_website_api/manager/ignore_suggestion_by_id?<token>&<suggestion_id>")]
fn ignoreSuggestionById(users: &State<Mutex<UserGroup>>, token: String, suggestion_id: String) -> Json<CommonResponse>{
    let common_response = CommonResponse::new(0);
    Json(common_response)
}


// ================= user ==============================
#[post("/image_processing_website_api/manager/get_all_user_msg?<token>")]
fn getAllUserMsg(token: String) -> Json<UserResponse>{
    let user_response = UserResponse::new(0, vec![UserMsg::new(0, 0, 0, 0, "".to_string())]);
    Json(user_response)
}

#[post("/image_processing_website_api/manager/erase_user_msg?<token>&<username>")]
fn eraseUserMsg(users: &State<Mutex<UserGroup>>, token: String, username: String) -> Json<CommonResponse>{
    let common_response = CommonResponse::new(0);
    Json(common_response)
}

// ================= history_operation ==============================
#[post("/image_processing_website_api/manager/get_all_history_operation?<token>")]
fn getAllHistoryOperation(users: &State<Mutex<UserGroup>>, token: String) -> Json<HistoryOperationResponse>{
    let history_operation_response =
        HistoryOperationResponse::new(vec![HistoryOperation::new("".to_string(), vec![], "".to_string(), Option::from(vec![]), 0)], 0);
    Json(history_operation_response)

}

pub fn get_routes() -> Vec<Route>{
    return routes![getAllSuggestions, submitResponseToSuggestionById, ignoreSuggestionById, getAllUserMsg, eraseUserMsg, getAllHistoryOperation]
}