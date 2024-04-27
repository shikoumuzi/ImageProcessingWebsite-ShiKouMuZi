use std::sync::Mutex;
use rocket::{Route, State};

use crate::backend_api::base_method::base::verifyToken;
use rocket::serde::json::Json;
use crate::backend_api::operation::utils::numeric_calculation::numeric_calculation::{NumericCalculation, Scalar};
use crate::typings::user::user::UserGroup;
use super::super::super::response::operation::image_index::Response as ImageIndexResponse;

#[get("/image_processing_website_api/operation/numeric_calculation/add_between_mats?<token>&<img_a>&<img_b>")]
async fn addBetweenMats(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, img_b: i32)-> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if (img_a < 0) || (img_b < 0) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.addBetweenMats(img_a, img_b);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/add_between_mat_and_value?<token>&<img_a>&<value>")]
async fn addBetweenMatAndValue(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, value: u8)-> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if img_a < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.addBetweenMatAndValue(img_a, value);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    println!("{}", dst_mat_index);
    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/add_between_mat_and_scalar?<token>&<img_a>&<r>&<g>&<b>")]
async fn addBetweenMatAndScalar(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, r: u8, g: u8, b:u8)-> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if img_a < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let scalar = Scalar{r, g , b};
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.addBetweenMatAndScalar(img_a, &scalar);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/sub_between_mats?<token>&<img_a>&<img_b>")]
async fn subBetweenMats(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, img_b: i32)-> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if (img_a < 0) || (img_b < 0) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.subBetweenMats(img_a, img_b);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/sub_between_mat_and_value?<token>&<img_a>&<value>")]
async fn subBetweenMatAndValue(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, value: u8)-> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if img_a < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.subBetweenMatAndValue(img_a, value);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/sub_between_mat_and_scalar?<token>&<img_a>&<r>&<g>&<b>")]
async fn subBetweenMatAndScalar(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, r: u8, g: u8, b:u8)-> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if img_a < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let scalar = Scalar{r, g , b};
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.subBetweenMatAndScalar(img_a, &scalar);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/bitwise_and?<token>&<img_a>&<img_b>")]
async fn bitwiseAnd(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, img_b: i32)-> Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if (img_a < 0) || (img_b < 0) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.bitwiseAnd(img_a, img_b);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/bitwise_or?<token>&<img_a>&<img_b>")]
async fn bitwiseOr(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, img_b: i32)-> Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if (img_a < 0) || (img_b < 0) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.bitwiseOr(img_a, img_b);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/bitwise_not?<token>&<img_a>&<img_b>")]
async fn bitwiseNot(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, img_b: i32)-> Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if (img_a < 0) || (img_b < 0) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.bitwiseNot(img_a, img_b);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/numeric_calculation/bitwise_xor?<token>&<img_a>&<img_b>")]
async fn bitwiseXor(users: &State<Mutex<UserGroup>>, token: String, img_a: i32, img_b: i32)-> Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if (img_a < 0) || (img_b < 0) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    
    let mut numeric_calculation_method: NumericCalculation = NumericCalculation{};
    let dst_mat_index = numeric_calculation_method.bitwiseXor(img_a, img_b);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

   let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

pub fn get_routes() -> Vec<Route>{
    return routes![
        addBetweenMats, addBetweenMatAndValue, addBetweenMatAndScalar,
        subBetweenMats, subBetweenMatAndValue, subBetweenMatAndScalar,
        bitwiseOr, bitwiseXor, bitwiseNot, bitwiseAnd]
}