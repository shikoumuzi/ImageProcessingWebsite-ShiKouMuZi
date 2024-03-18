use std::sync::Mutex;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;

#[get("/image_processing_website_api/about?<token>&<target_content_title>")]
fn about(token: String, target_content_title: String){


}