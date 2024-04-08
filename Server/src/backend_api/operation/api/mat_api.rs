use std::ffi::{CString, OsStr};
use std::sync::Mutex;
use rocket::{Route, State};
use crate::sqlite::sqlite::SQLite;
use crate::typings::user::user::UserGroup;
use super::super::utils::mat;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::Data;
use rocket::data::FromData;
use rocket::form::{Form, FromForm};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};
use crate::backend_api::base_method::base::verifyToken;
use super::super::super::response::operation::{common::Response as CommonResponse, mat::read_img::Response as ReadImgResponse, image_index::Response as ImageIndexResponse};
use super::typngs::Image::Image;
use std::fs;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use std::option::Option;
use super::super::super::base_method::base::USER_IMG_PATH;
use super::super::utils::mat::mat::Mat;
use super::base_method::*;


#[post("/image_processing_website_api/operation/mat/read_img", data="<form>")]
fn readImg(users: &State<Mutex<UserGroup>>, form: Form<Image<'_>>, content_type: &ContentType)-> Json<ReadImgResponse>{


    let _user = verifyToken(&users, &form.token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let response = ReadImgResponse::new(1, -1);
        return Json(response);
    }

    // let user_img_path_str = USER_IMG_PATH.to_string();
    // let tmp_user_img_path = Path::new(&user_img_path_str);
    // let file_extenion: String = get_extension_from_filename(form.file_name.as_str()).unwrap().to_string();
    //
    // let user_img_path_buf = tmp_user_img_path.join(&_user.as_ref().unwrap().username)
    //                                      .join(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string() + "." + file_extenion.as_str());
    //
    // let mut user_img_path: &Path = user_img_path_buf.as_path();
    // unsafe { std::fs::copy(form.file.path().unwrap(), user_img_path); }

    let path_buf = self::saveFileToUserStoreByForm(&form, _user.unwrap().username);
    let mut mat_method: Mat = Mat{};
    let mat_index = mat_method.readImg(path_buf.to_str().unwrap(), 1);
    if mat_index < 0{
        let response = ReadImgResponse::new(1, -1);
        return Json(response);
    }

    let response = ReadImgResponse::new(0, mat_index);
    Json(response)

}

#[get("/image_processing_website_api/operation/mat/save_img?<token>&<mat_index>")]
async fn saveImg(users: &State<Mutex<UserGroup>>, token: String, mat_index: i32) -> std::option::Option<NamedFile>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        return Option::None;
    }
    if mat_index < 0{
        return Option::None
    }
    let path_buf = self::saveFileToUserStore(_user.unwrap().username);
    let mut mat_method: Mat = Mat{};
    mat_method.saveImg(mat_index, path_buf.to_str().unwrap());

    return std::option::Option::from(NamedFile::open(path_buf).await.ok()?);
}

#[get("/image_processing_website_api/operation/mat/free_img?<token>&<mat_index>")]
fn freeImg(users: &State<Mutex<UserGroup>>, token: String, mat_index: i32) -> Json<CommonResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let response = CommonResponse::new(1);
        return Json(response);
    }
    if mat_index < 0{
        let response = CommonResponse::new(1);
        return Json(response);
    }
    let path_buf = self::saveFileToUserStore(_user.unwrap().username);
    let mut mat_method: Mat = Mat{};
    mat_method.freeImg(mat_index);

    let response = CommonResponse::new(0);
    return Json(response);
}

#[get("/image_processing_website_api/operation/mat/copy?<token>&<src_mat_index>&<dst_mat_index>")]
async fn copy(users: &State<Mutex<UserGroup>>, token: String, src_mat_index: i32, dst_mat_index: i32) -> Json<ImageIndexResponse> {
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    if (src_mat_index < 0) || (dst_mat_index < 0){
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    let mut mat_method: Mat = Mat{};
    mat_method.copy(src_mat_index, dst_mat_index);

    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}
#[get("/image_processing_website_api/operation/mat/hstack?<token>&<mat_index_vec>")]
async fn hstack(users: &State<Mutex<UserGroup>>, token: String, mat_index_vec: Vec<i32>) -> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    for i in 0..mat_index_vec.len(){
        if mat_index_vec[i] < 0{
            let image_index_response = ImageIndexResponse::new(1, -1);
            return Json(image_index_response);
        }
    }

    let mut mat_method: Mat = Mat{};
    let dst_mat_index = mat_method.hstack(mat_index_vec.as_ptr(), mat_index_vec.len() as u32);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/mat/vstack?<token>&<mat_index_vec>")]
async fn vstack(users: &State<Mutex<UserGroup>>, token: String, mat_index_vec: Vec<i32>) -> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    for i in 0..mat_index_vec.len(){
        if mat_index_vec[i] < 0{
            let image_index_response = ImageIndexResponse::new(1, -1);
            return Json(image_index_response);
        }
    }

    let mut mat_method: Mat = Mat{};
    let dst_mat_index = mat_method.vstack(mat_index_vec.as_ptr(), mat_index_vec.len() as u32);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}

#[get("/image_processing_website_api/operation/mat/resize?<token>&<mat_index>&<width>&<height>")]
async fn resize(users: &State<Mutex<UserGroup>>, token: String, mat_index: i32, width: u32, height: u32) -> Json<ImageIndexResponse>{
    let _user = verifyToken(&users, &token);
    if (_user.as_ref().is_none()) || (_user.as_ref().unwrap().authority != 1) {
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    if mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }

    let mut mat_method: Mat = Mat{};
    let dst_mat_index = mat_method.resize(mat_index, width, height);
    if dst_mat_index < 0{
        let image_index_response = ImageIndexResponse::new(1, -1);
        return Json(image_index_response);
    }
    let image_index_response = ImageIndexResponse::new(0, dst_mat_index);
    return Json(image_index_response);
}
pub fn get_routes() -> Vec<Route>{
    return routes![readImg, saveImg, freeImg, copy, hstack, vstack, resize]
}