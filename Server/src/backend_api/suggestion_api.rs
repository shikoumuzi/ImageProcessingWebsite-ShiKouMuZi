use std::sync::Mutex;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use crate::typings::user::user::UserGroup;
use super::response::suggestion::suggestion::Response as SuggestionResponse;

#[get("/image_processing_website_api/submit_suggestion?<token>&<content>&<time_stamp>")]
fn submitSuggestion(users: &State<Mutex<UserGroup>>, token: String, content: String, time_stamp: u64) -> Json<SuggestionResponse>{
    let suggestion_response = SuggestionResponse::new(0);
    Json(suggestion_response)
}

pub fn get_routes() -> Vec<Route>{
    return routes![submitSuggestion]
}