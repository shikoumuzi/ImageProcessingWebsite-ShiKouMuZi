use std::sync::Mutex;
use std::vec;
use std::option::Option;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;


pub fn get_routes() -> Vec<Route>{
    return routes![]
}