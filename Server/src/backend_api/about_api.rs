use std::sync::Mutex;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use crate::backend_api::response::about::about::Response as AboutResponse;

#[get("/image_processing_website_api/about?<token>&<target_content_title>")]
fn about(token: String, target_content_title: String)->Json<AboutResponse>{

    let about_response = AboutResponse::new(vec![], "".to_string(), vec![], vec![], 0);
    Json(about_response)
}

pub fn get_routes()-> Vec<Route> {
    return routes![self::about]
}