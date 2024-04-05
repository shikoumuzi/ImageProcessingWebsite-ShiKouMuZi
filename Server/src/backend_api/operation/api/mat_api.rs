use std::ffi::OsStr;
use std::sync::Mutex;
use rocket::State;
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
use super::super::super::response::operation::{common::Response as CommonResponse, mat::read_img::Response as ReadImgResponse};
use super::typngs::Image::Image;
use std::fs;
use std::path::Path;
use std::ptr::copy;
use super::super::super::base_method::base::USER_IMG_PATH;
use super::super::utils::mat::mat::Mat;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn svaeFileToUserStore<'f>(form: &'f Form<Image<'_>>, username: String){
    let user_img_path_str = USER_IMG_PATH.to_string();
    let tmp_user_img_path = Path::new(&user_img_path_str);
    let file_extenion: String = get_extension_from_filename(form.file_name.as_str()).unwrap().to_string();

    let user_img_path_buf = tmp_user_img_path.join(username)
        .join(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string() + "." + file_extenion.as_str());

    let mut user_img_path: &Path = user_img_path_buf.as_path();
    unsafe { std::fs::copy(form.file.path().unwrap(), user_img_path); }
}


#[post("/image_processing_website_api/operation/mat/read_img", data="<form>")]
pub fn readImg(users: &State<Mutex<UserGroup>>, form: Form<Image<'_>>, content_type: &ContentType)-> Json<ReadImgResponse>{


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

    self::svaeFileToUserStore(&form, _user.unwrap().username);

    let mat_index =

    let response = ReadImgResponse::new(1, -1);
    Json(response)

}
// #[post("/image_processing_website_api/operation/mat/read_img", data="<form>")]
// pub fn saveImg(users: &State<Mutex<UserGroup>>, form: Form<Image<'_>>, content_type: &ContentType) -> Json<CommonResponse>{
//
// }
