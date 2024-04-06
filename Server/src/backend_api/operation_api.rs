use std::sync::Mutex;
use std::vec;
use std::option::Option;
use super::super::typings::user::user::User;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;

use super::operation::api::{mat_api, affine_transform_api, numeric_calculation_api};


pub fn get_routes() -> Vec<Route>{
    let mut routes = routes![];
    routes.append(&mut mat_api::get_routes());
    routes.append(&mut affine_transform_api::get_routes());
    routes.append(&mut numeric_calculation_api::get_routes());

    return routes
}