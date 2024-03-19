use std::sync::Mutex;
use std::vec;
use std::option::Option;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use crate::typings::user::user::UserGroup;
use super::response::history_operation::earse_history_operation::Response as EarseHistoryOperationResponse;
use super::response::history_operation::get_history_operation::Response as GetHistoryOperationResponse;
use super::response::history_operation::history_operation_details::Response as HistoryOperationDetailsResponse;


#[get("/image_processing_website_api/history_operations?<token>&<username>")]
fn getHistoryOperations(users: &State<Mutex<UserGroup>>, token: String, username: String) -> Json<GetHistoryOperationResponse>{
    let get_history_operation_response = GetHistoryOperationResponse::new(vec![], 0);
    Json(get_history_operation_response)

}

#[post("/image_processing_website_api/earse_history_operation?<token>&<history_operation_id>")]
fn earseHistoryOperation(users: &State<Mutex<UserGroup>>, token: String,  history_operation_id: String) ->Json<EarseHistoryOperationResponse>{
    let earse_history_operation_response = EarseHistoryOperationResponse::new(0);
    Json(earse_history_operation_response)
}

#[post("/image_processing_website_api/get_operation_details_by_history_operation_id?<token>&<history_operation_id>")]
fn getOperationDetailsByHistoryOperationId(users: &State<Mutex<UserGroup>>, token: String,  history_operation_id: String) -> Json<HistoryOperationDetailsResponse> {
    let history_operation_response = HistoryOperationDetailsResponse::new(vec![], 0);
    Json(history_operation_response)
}


pub fn get_routes() -> Vec<Route>{
    return routes![getHistoryOperations, getOperationDetailsByHistoryOperationId, earseHistoryOperation]
}