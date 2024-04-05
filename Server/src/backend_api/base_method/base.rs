use std::sync::Mutex;
use std::vec;
use std::option::Option;
use std::path::Path;
use std::string::ToString;
use crate::typings::user::user::{User, UserGroup};

use rocket::fairing::AdHoc;
use rocket::{Route, State};

pub const USER_IMG_PATH: &str ="F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Store\\user_store";

pub fn verifyToken<'a>(users: &'a State<Mutex<UserGroup>>, token: &String) -> Option<User> {
    let mut _users = users.lock().unwrap();
    let user = _users.find_user_by_token(token);
    if user.is_none(){
        return Option::None;
    }
    return Option::from(user.unwrap().clone());
}