use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use super::super::typings::user::user::User;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use rusqlite::params;
use crate::sqlite::sqlite::SQLite;
use crate::typings::user::user::UserGroup;
use super::response::suggestion::suggestion::Response as SuggestionResponse;

#[get("/image_processing_website_api/submit_suggestion?<token>&<content>&<time_stamp>")]
fn submitSuggestion(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String, content: String, time_stamp: u64) -> Json<SuggestionResponse>{

    let mut _users = users.lock().unwrap();
    let user = _users.find_user_by_token(&token);
    if user.is_none() {
        let suggestion_response = SuggestionResponse::new(1);
        return Json(suggestion_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();

    let execute_result =
        conn.execute("INSERT INTO SUGGESTION(USER_NAME, SUGGESTION_CONTENT, TIME_STAMP, SUGGECTION_STATE) VALUE (?1, ?2, ?3, ?5)",
                     params![user.unwrap().username, content, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()]);
    if execute_result.is_err(){
        println!("stmt is error {:?}", execute_result);
        let suggestion_response = SuggestionResponse::new(1);
        return Json(suggestion_response);
    }

    let suggestion_response = SuggestionResponse::new(0);
    Json(suggestion_response)
}

pub fn get_routes() -> Vec<Route>{
    return routes![submitSuggestion]
}