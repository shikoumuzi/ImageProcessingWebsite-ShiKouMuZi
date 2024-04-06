use std::ffi::{CString, OsStr};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::form::Form;
use crate::backend_api::base_method::base::USER_IMG_PATH;
use crate::backend_api::operation::api::typngs::Image::Image;

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn saveFileToUserStoreByForm<'f>(form: &'f Form<Image<'_>>, username: String)-> PathBuf{
    let user_img_path_str = USER_IMG_PATH.to_string();
    let tmp_user_img_path = Path::new(&user_img_path_str);
    let file_extenion: String = get_extension_from_filename(form.file_name.as_str()).unwrap().to_string();

    let user_img_path_buf = tmp_user_img_path.join(username)
        .join(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string() + "." + file_extenion.as_str());

    let mut user_img_path: &Path = user_img_path_buf.as_path();
    unsafe { std::fs::copy(form.file.path().unwrap(), user_img_path); }

    return user_img_path_buf
}

pub fn saveFileToUserStore(username: String)-> PathBuf{
    let user_img_path_str = USER_IMG_PATH.to_string();
    let tmp_user_img_path = Path::new(&user_img_path_str);

    let user_img_path_buf = tmp_user_img_path.join(username)
        .join(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string() + ".jpg");

    return user_img_path_buf
}

pub fn path2CString(path_buf: PathBuf) -> CString{
    let mut path = path_buf.as_os_str().to_str().unwrap().to_string().replace("\\", "/");
    println!("{}", path);
    let cpath = CString::new(path).unwrap();
    println!("path is {:?}", cpath);
    return cpath
}