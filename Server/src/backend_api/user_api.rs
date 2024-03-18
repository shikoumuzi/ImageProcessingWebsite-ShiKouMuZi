use std::sync::Mutex;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use crate::typings::user::user::UserGroup;
use crate::backend_api::response::user::login::Response as LoginResponse;
use crate::backend_api::response::user::common::Response as CommonResponse;


#[post("/image_processing_website_api/login?<username>&<password>")]
fn login(users: &State<Mutex<UserGroup>>, username: String, password: String)-> Json<LoginResponse> {
    
    
    let login_response = LoginResponse::new(0, 0, 0, "".to_string());
    Json(login_response)
}

#[post("/image_processing_website_api/register?<username>&<password>&<time_stamp>")]
fn register(users: &State<Mutex<UserGroup>>, username: String, password: String, time_stamp: u64) -> Json<CommonResponse>{

    let resigter_response = CommonResponse::new(0);
    Json(resigter_response)
}

#[post("/image_processing_website_api/resetPassword?<token>&<username>&<old_password>&<new_password>")]
fn resetPassword(users: &State<Mutex<UserGroup>>, token: String, username: String, old_password: String, new_password: String) -> Json<CommonResponse> {

    let resigter_response = CommonResponse::new(0);
    Json(resigter_response)
}

#[post("/image_processing_website_api/checkManagerAuthority?<token>")]
fn checkManagerAuthority(users: &State<Mutex<UserGroup>>, token: String)-> Json<CommonResponse> {

    let resigter_response = CommonResponse::new(0);
    Json(resigter_response)
}

#[post("/image_processing_website_api/checkPassword?<token>&<username>&<password>")]
fn checkPassword(users: &State<Mutex<UserGroup>>, token: String, username: String, password: String) -> Json<CommonResponse> {

    let resigter_response = CommonResponse::new(0);
    Json(resigter_response)
}

pub fn get_routes() -> Vec<Route> {
    return routes![self::login, self::register, self::resetPassword, self::checkManagerAuthority, self::checkPassword]
}