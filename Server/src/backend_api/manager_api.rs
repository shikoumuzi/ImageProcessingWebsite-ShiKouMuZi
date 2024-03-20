use std::sync::Mutex;
use std::vec;
use std::option::Option;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use rusqlite::ffi::sqlite3;
use rusqlite::params;
use crate::sqlite::sqlite::SQLite;
// use serde::de::Unexpected::Option;
use crate::typings::user::user::UserGroup;
use super::response::manager::common::Response as CommonResponse;
use super::response::manager::suggestion::Response as SuggestionResponse;
use super::response::manager::suggestion::Suggestion;
use super::response::manager::user::{Response as UserResponse, UserMsg};
use super::response::manager::history_operation::{HistoryOperation, Response as HistoryOperationResponse};
use super::base_method::base_method::{verifyToken};
// ================= suggestion ==============================

#[post("/image_processing_website_api/manager/get_all_suggestions?<token>&<now_len>")]
fn getAllSuggestions(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String, now_len: u16) -> Json<SuggestionResponse>{
    let user = verifyToken(users, &token);
    if (user.as_ref().is_none()) || (user.as_ref().unwrap().authority != 2) || (now_len > 100){
        let suggestion_response = SuggestionResponse::new(1, vec![]);
        return Json(suggestion_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();
    let sql_lang = format!("SELECT SUGGECTION_ID, TIME_STAMP, USER_NAME, SUGGESTION_CONTENT FROM SUGGESTION WHERE RESPONSE IS NULL LIMIT {}, {}", now_len, 100);
    println!("{}", sql_lang);
    let stmt_result = conn.prepare(sql_lang.as_str());
    if stmt_result.is_err(){
        println!("stmt is error {:?}", stmt_result);
        let suggestion_response = SuggestionResponse::new(1, vec![]);
        return Json(suggestion_response);
    }

    let mut stmt = stmt_result.unwrap();
    let suggestions_result = stmt.query_map(params![], |row| {
        Ok(Suggestion{
            content: row.get(3)?,
            suggestion_id: row.get(0)?,
            time_stamp: row.get(1)?,
            username: row.get(2)?,
        })
    });

    if suggestions_result.is_err(){
        println!("suggestions_result is error ");
        let suggestion_response = SuggestionResponse::new(1, vec![]);
        return Json(suggestion_response);
    }
    let suggestions = suggestions_result.unwrap();
    let mut suggestion_vec = vec![];
    for suggestion in suggestions{
        suggestion_vec.push(suggestion.unwrap());
    }

    let suggestion_response = SuggestionResponse::new(0, suggestion_vec);
    Json(suggestion_response)
}

#[post("/image_processing_website_api/manager/submit_response_to_suggestion_by_id?<token>&<suggestion_id>&<response>")]
fn submitResponseToSuggestionById(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String, suggestion_id: u64, response: String) -> Json<CommonResponse>{
    let user = verifyToken(&users, &token);
    if (user.as_ref().is_none()) || (user.as_ref().unwrap().authority != 2) {
        let common_response = CommonResponse::new(1);
        return Json(common_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();
    let execute_result = conn.execute("UPDATE SUGGESTION SET RESPONSE=?1 WHERE SUGGESTION_ID=?2", params![response, suggestion_id]);
    if execute_result.is_err(){
        println!("execute is error {:?}", execute_result);
        let common_response = CommonResponse::new(1);
        return Json(common_response);
    }

    let common_response = CommonResponse::new(0);
    Json(common_response)
}

#[post("/image_processing_website_api/manager/ignore_suggestion_by_id?<token>&<suggestion_id>")]
fn ignoreSuggestionById(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String, suggestion_id: String) -> Json<CommonResponse>{
    let user = verifyToken(&users, &token);
    if (user.as_ref().is_none()) || (user.as_ref().unwrap().authority != 2) {
        let common_response = CommonResponse::new(1);
        return Json(common_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();
    let execute_result = conn.execute("DELETE FROM SUGGESTION WHERE SUGGESTION_ID=?1", params![suggestion_id]);

    if execute_result.is_err(){
        println!("execute is error {:?}", execute_result);
        let common_response = CommonResponse::new(1);
        return Json(common_response);
    }

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