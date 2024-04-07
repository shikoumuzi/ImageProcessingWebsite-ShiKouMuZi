use std::sync::Mutex;
use rocket::{Route, State};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use crate::backend_api::base_method::base::verifyToken;
use crate::backend_api::operation::utils::affine_transform::affine_transform::FLIP_CODE;
use crate::typings::user::user::UserGroup;
use super::super::utils::affine_transform::affine_transform::AffineTransForm;
use super::base_method::*;
use super::super::super::response::operation::image_index::Response as ImageIndexResponse;

#[get("/image_processing_website_api/operation/affine_transform/leftRotate90?<token>&<mat>")]
async fn leftRotate90(users: &State<Mutex<UserGroup>>, token: String, mat: i32)-> Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
                let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if mat < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    let mut affine_transform: AffineTransForm = AffineTransForm{};
    let dst_mat_index = affine_transform.leftRotate90(mat);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/affine_transform/rightRotate90?<token>&<mat>")]
async fn rightRotate90(users: &State<Mutex<UserGroup>>, token: String, mat: i32)-> Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
                let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if mat < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    let mut affine_transform: AffineTransForm = AffineTransForm{};
    let dst_mat_index = affine_transform.rightRotate90(mat);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/affine_transform/flip?<token>&<mat>&<flip_code>")]
async fn flip(users: &State<Mutex<UserGroup>>, token: String, mat: i32, flip_code: i8)->Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
                let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if mat < 0{
                let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    let mut affine_transform: AffineTransForm = AffineTransForm{};
    let dst_mat_index = affine_transform.flip(mat, flip_code);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

pub fn get_routes() -> Vec<Route>{
    return routes![
        leftRotate90,
        rightRotate90,
        flip
    ]
}