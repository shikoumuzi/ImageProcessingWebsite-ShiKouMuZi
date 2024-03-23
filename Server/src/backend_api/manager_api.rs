use std::sync::Mutex;
use std::vec;
use std::option::Option;
use std::path::Path;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use rusqlite::ffi::sqlite3;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use crate::sqlite::sqlite::SQLite;
// use serde::de::Unexpected::Option;
use crate::typings::user::user::UserGroup;
use super::response::manager::common::Response as CommonResponse;
use super::response::manager::suggestion::Response as SuggestionResponse;
use super::response::manager::suggestion::Suggestion;
use super::response::manager::user::{Response as UserResponse, UserMsg};
use super::response::manager::history_operation::{HistoryOperation, Operation, Response as HistoryOperationResponse};
use super::base_method::base::{USER_IMG_PATH, verifyToken};
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
fn getAllUserMsg(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String) -> Json<UserResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 2) {
        let UserResponse = UserResponse::new(1, vec![]);
        return Json(UserResponse);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();
    let user_stmt_result = conn.prepare("SELECT USER_NAME, AUTHORITY, TIME_STAMP FROM USERS");
    if user_stmt_result.is_err(){
        println!("stmt is error {:?}", user_stmt_result);
        let UserResponse = UserResponse::new(1, vec![]);
        return Json(UserResponse);
    }

    let mut user_stmt = user_stmt_result.unwrap();
    let users_result = user_stmt.query_map(params![], |row| {
        Ok(UserMsg{
            authority: row.get(1)?,
            history_operation_count: 0,
            result_image_count: 0,
            time_stamp: row.get(2)?,
            username: row.get(0)?
        })
    });

    if users_result.is_err() {
        println!("users_result is error");
        let UserResponse = UserResponse::new(1, vec![]);
        return Json(UserResponse);
    }

    let users = users_result.unwrap();
    let mut response_usermsg: Vec<UserMsg> = vec![];

    let user_base_img_str: String = USER_IMG_PATH.to_string();
    let user_base_img_path: &Path = Path::new(&user_base_img_str);

    for user in users{
        if user.is_err(){
            println!("user is error");
            continue
        }
        let mut tmp_user = user.unwrap().clone();

        let user_img_path = user_base_img_path.join(&tmp_user.username);
        if !user_img_path.exists(){
            let create_result = std::fs::create_dir(&user_img_path);
            if create_result.is_err() {
                println!("create dir is error {:?}", create_result);
                continue;
            }
        }

        let entry = std::fs::read_dir(user_img_path);
        tmp_user.result_image_count = entry.unwrap().count() as u64;

        let history_operation_stmt_result = conn.prepare(format!("SELECT COUNT(*) FROM HISTORY_OPERATION WHERE USER_NAME='{}'", tmp_user.username).as_str());
        if history_operation_stmt_result.is_err(){
            println!("history_operation_stmt is error");
            continue
        }

        let mut history_operation_stmt = history_operation_stmt_result.unwrap();
        tmp_user.history_operation_count = history_operation_stmt.query_map(params![], |row| row.get(0)?);
        response_usermsg.push(tmp_user);
    }

    let user_response = UserResponse::new(0, response_usermsg);
    Json(user_response)
}

#[post("/image_processing_website_api/manager/erase_user_msg?<token>&<username>")]
fn eraseUserMsg(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String, username: String) -> Json<CommonResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 2) || (_user.as_ref().unwrap().username == username){
        let common_response = CommonResponse::new(1);
        return Json(common_response);
    }

    let mut _users = users.lock().unwrap();
    _users.erase_user(&username);

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();
    let execute_result = conn.execute("DELETE FROM USERS WHERE USER_NAME=?1", params![username]);

    if execute_result.is_err(){
        println!("execute is error");
        let common_response = CommonResponse::new(1);
        return Json(common_response);
    }

    let common_response = CommonResponse::new(0);
    Json(common_response)
}

// ================= history_operation ==============================
struct HistoryOperationRow{
    pub operation_id: String,
    pub operation_details: String,
    pub note:String,
    pub time_stamp: u64
}

#[derive(Deserialize, Serialize)]
struct Operations{
    pub operation_detals: Vec<Operation>
}

impl Operations {
    pub fn new() -> Operations {
        return Operations{
            operation_detals: vec![]
        }
    }
}

#[post("/image_processing_website_api/manager/get_all_history_operation?<token>")]
fn getAllHistoryOperation(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String) -> Json<HistoryOperationResponse>{
    let user = verifyToken(&users, &token);
    if (user.as_ref().is_none()) || (user.as_ref().unwrap().authority != 2) {
        let history_operation_response =
            HistoryOperationResponse::new(vec![], 1);
        return Json(history_operation_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();

    let stmt_result = conn.prepare("SELECT OPERATION_ID, OPERATOR_DETAILS, NOTE, TIME_STAMP FROM HISTORY_OPERATION");
    if stmt_result.is_err(){
        println!("stmt is error");
        let history_operation_response =
            HistoryOperationResponse::new(vec![], 1);
        return Json(history_operation_response);
    }

    let mut stmt = stmt_result.unwrap();
    let history_operations_result = stmt.query_map(params![], |row|{
        Ok(HistoryOperationRow{
            operation_id: row.get(0)?,
            operation_details: row.get(1)?,
            note: row.get(2)?,
            time_stamp: row.get(3)?
        })
    });

    if history_operations_result.is_err(){
        println!("history_operation is error");
        let history_operation_response =
            HistoryOperationResponse::new(vec![], 1);
        return Json(history_operation_response);
    }

    let history_operations = history_operations_result.unwrap();
    let mut history_operation_vec: Vec<HistoryOperation> = vec![];
    for history_operation in history_operations{
        if history_operation.is_err(){
            continue;
        }
        let unwrap_history_operation_row = history_operation.unwrap();
        let mut tmp_history_operation = HistoryOperation::new("".to_string(), vec![], "".to_string(), Option::from(vec![]), 0);
        tmp_history_operation.history_operation_id = unwrap_history_operation_row.operation_id;
        tmp_history_operation.note = unwrap_history_operation_row.note;
        tmp_history_operation.time_stamp = unwrap_history_operation_row.time_stamp;
        let mut operations: Operations = rocket::serde::json::from_str(unwrap_history_operation_row.operation_details.as_str()).unwrap();
        if operations.operation_detals.len() == 0{
            tmp_history_operation.operations = Option::None;
        }else {
            tmp_history_operation.operations = Option::from(operations.operation_detals);
        }

    }

    let history_operation_response =
        HistoryOperationResponse::new(history_operation_vec, 0);
    Json(history_operation_response)

}

pub fn get_routes() -> Vec<Route>{
    return routes![getAllSuggestions, submitResponseToSuggestionById, ignoreSuggestionById, getAllUserMsg, eraseUserMsg, getAllHistoryOperation]
}