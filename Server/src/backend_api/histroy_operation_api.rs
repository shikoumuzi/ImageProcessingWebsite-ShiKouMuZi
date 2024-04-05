use std::fmt::format;
use std::sync::Mutex;
use std::vec;
use std::option::Option;
use super::super::typings::user::user::User;
use rocket::fairing::AdHoc;
use rocket::{execute, Route, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rusqlite::params;
use crate::backend_api::base_method::base::verifyToken;
use crate::sqlite::sqlite::SQLite;
use crate::typings::user::user::UserGroup;
use super::response::history_operation::earse_history_operation::Response as EarseHistoryOperationResponse;
use super::response::history_operation::get_history_operation::{Response as GetHistoryOperationResponse, HistoryOperation};
use super::response::history_operation::history_operation_details::{Response as HistoryOperationDetailsResponse, Operation};

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


#[get("/image_processing_website_api/history_operations?<token>&<username>")]
fn getHistoryOperations(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>,token: String, username: String) -> Json<GetHistoryOperationResponse> {

    let user = verifyToken(&users, &token);
    if (user.as_ref().is_none()) || (user.as_ref().unwrap().authority != 1) {
        let get_history_operation_response = GetHistoryOperationResponse::new(vec![], 1);
        return Json(get_history_operation_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();

    let stmt_result = conn.prepare(format!("SELECT OPERATION_ID, OPERATOR_DETAILS, NOTE, TIME_STAMP FROM HISTORY_OPERATION WHERE USER_NAME='{}'", username).as_str());
    if stmt_result.is_err(){
        println!("stmt is error");
        let get_history_operation_response = GetHistoryOperationResponse::new(vec![], 1);
        return Json(get_history_operation_response);
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
        let get_history_operation_response = GetHistoryOperationResponse::new(vec![], 1);
        return Json(get_history_operation_response);
    }

    let history_operations = history_operations_result.unwrap();
    let mut history_operation_vec: Vec<HistoryOperation> = vec![];
    for history_operation in history_operations{
        if history_operation.is_err(){
            continue;
        }
        let unwrap_history_operation_row = history_operation.unwrap();
        let mut tmp_history_operation = HistoryOperation::new("".to_string(), "".to_string(), 0);
        tmp_history_operation.history_operation_id = unwrap_history_operation_row.operation_id;
        tmp_history_operation.note = unwrap_history_operation_row.note;
        tmp_history_operation.time_stamp = unwrap_history_operation_row.time_stamp;
        history_operation_vec.push(tmp_history_operation);
    }


    let get_history_operation_response = GetHistoryOperationResponse::new(history_operation_vec, 0);
    Json(get_history_operation_response)

}

#[post("/image_processing_website_api/earse_history_operation?<token>&<history_operation_id>")]
fn earseHistoryOperation(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String,  history_operation_id: String) ->Json<EarseHistoryOperationResponse>{

    let user = verifyToken(&users, &token);
    if (user.as_ref().is_none()) || (user.as_ref().unwrap().authority != 1) {
        let earse_history_operation_response = EarseHistoryOperationResponse::new( 1);
        return Json(earse_history_operation_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();

    let execute_result = conn.execute("DELETE FROM HISTORY_OPERATION WHERE OPERATION_ID=?1", params![history_operation_id]);
    if execute_result.is_err(){
        println!("execute is error {:?}", execute_result);
        let earse_history_operation_response = EarseHistoryOperationResponse::new( 1);
        return Json(earse_history_operation_response);
    }

    let earse_history_operation_response = EarseHistoryOperationResponse::new(0);
    Json(earse_history_operation_response)
}

struct OperationDetailsString{
    operations: String
}

#[post("/image_processing_website_api/get_operation_details_by_history_operation_id?<token>&<history_operation_id>")]
fn getOperationDetailsByHistoryOperationId(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String,  history_operation_id: String) -> Json<HistoryOperationDetailsResponse> {

    let user = verifyToken(&users, &token);
    if (user.as_ref().is_none()) || (user.as_ref().unwrap().authority != 1) {
        let history_operation_detail_response = HistoryOperationDetailsResponse::new( vec![], 1);
        return Json(history_operation_detail_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();

    let stmt_result = conn.prepare(format!("SELECT OPERAION_DETAILS FROM HISTORY_OPERATION WHERE OPERATION_ID='{}'", history_operation_id).as_str());
    if stmt_result.is_err(){
        println!("stmt is error {:?}", stmt_result);
        let history_operation_detail_response = HistoryOperationDetailsResponse::new( vec![], 1);
        return Json(history_operation_detail_response);
    }

    let mut stmt = stmt_result.unwrap();
    let details_result = stmt.query_map(params![], |row| {
        Ok(
            OperationDetailsString{
                operations: row.get(0)?
            }
        )
    });
    if details_result.is_err(){
        println!("details_result is error");
        let history_operation_detail_response = HistoryOperationDetailsResponse::new( vec![], 1);
        return Json(history_operation_detail_response);
    }

    let details = details_result.unwrap();
    let mut operation_details_vec :Vec<Operation> = vec![];
    for detail in details{
        if detail.is_err(){
            println!("detail is error");
            let history_operation_detail_response = HistoryOperationDetailsResponse::new( vec![], 1);
            return Json(history_operation_detail_response);
        }
        let unwrap_detail = detail.unwrap();
        let operations: Operations = rocket::serde::json::from_str(unwrap_detail.operations.as_str()).unwrap();
        operation_details_vec = operations.operation_detals;
        let history_operation_response = HistoryOperationDetailsResponse::new(operation_details_vec, 0);
        return Json(history_operation_response);
    }

    let history_operation_response = HistoryOperationDetailsResponse::new(vec![], 0);
    Json(history_operation_response)
}


pub fn get_routes() -> Vec<Route>{
    return routes![getHistoryOperations, getOperationDetailsByHistoryOperationId, earseHistoryOperation]
}