use std::sync::Mutex;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use crate::typings::user::user::UserGroup;
use crate::backend_api::response::user::login::Response as LoginResponse;
use crate::backend_api::response::user::resigter::Response as ResigterResponse;


#[post("/image_processing_website_api/login?<username>&<password>")]
fn login(users: &State<Mutex<UserGroup>>, username: String, password: String)-> Json<LoginResponse> {
    
    
    let login_response = LoginResponse::new(0, 0, 0, "".to_string());
    Json(login_response)
}

#[post("/image_processing_website_api/register?<username>&<password>&<time_stamp>")]
fn register(users: &State<Mutex<UserGroup>>, username: String, password: String, time_stamp: u64) -> Json<ResigterResponse>{

    let resigter_response = ResigterResponse::new(0);
    Json(resigter_response)
}

pub fn get_routes() -> Vec<Route> {
    return routes![self::login, self::register]
}