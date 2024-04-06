use std::sync::Mutex;
use rocket::{Route, State};
use rocket::fs::NamedFile;

use crate::backend_api::base_method::base::verifyToken;
use crate::backend_api::operation::utils::affine_transform::affine_transform::FLIP_CODE;
use crate::typings::user::user::UserGroup;
use super::super::utils::affine_transform::affine_transform::AffineTransForm;
use super::base_method::*;
#[post("/image_processing_website_api/operation/affine_transform/leftRotate90?<token>&<mat>")]
async fn leftRotate90(users: &State<Mutex<UserGroup>>, token: String, mat: i32)-> Option<NamedFile> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        return Option::None;
    }
    if mat < 0{
        return Option::None;
    }
    let path_buf = self::saveFileToUserStore(_user.unwrap().username);
    let mut affine_transform: AffineTransForm = AffineTransForm{};
    let dst_mat_index = affine_transform.leftRotate90(mat);
    if dst_mat_index < 0{
        return Option::None;
    }

    affine_transform.saveImg(dst_mat_index, path_buf.to_str().unwrap());
    return std::option::Option::from(NamedFile::open(path_buf).await.ok()?);
}

#[post("/image_processing_website_api/operation/affine_transform/rightRotate90?<token>&<mat>")]
async fn rightRotate90(users: &State<Mutex<UserGroup>>, token: String, mat: i32)-> Option<NamedFile> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        return Option::None;
    }
    if mat < 0{
        return Option::None;
    }
    let path_buf = self::saveFileToUserStore(_user.unwrap().username);
    let mut affine_transform: AffineTransForm = AffineTransForm{};
    let dst_mat_index = affine_transform.rightRotate90(mat);
    if dst_mat_index < 0{
        return Option::None;
    }

    affine_transform.saveImg(dst_mat_index, path_buf.to_str().unwrap());
    return std::option::Option::from(NamedFile::open(path_buf).await.ok()?);
}

#[post("/image_processing_website_api/operation/affine_transform/flip?<token>&<mat>&<flip_code>")]
async fn flip(users: &State<Mutex<UserGroup>>, token: String, mat: i32, flip_code: i8)-> Option<NamedFile> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        return Option::None;
    }
    if mat < 0{
        return Option::None;
    }
    let path_buf = self::saveFileToUserStore(_user.unwrap().username);
    let mut affine_transform: AffineTransForm = AffineTransForm{};
    let dst_mat_index = affine_transform.flip(mat, flip_code);
    if dst_mat_index < 0{
        return Option::None;
    }

    affine_transform.saveImg(dst_mat_index, path_buf.to_str().unwrap());
    return std::option::Option::from(NamedFile::open(path_buf).await.ok()?);
}

pub fn get_routes() -> Vec<Route>{
    return routes![
        leftRotate90,
        rightRotate90,
        flip
    ]
}