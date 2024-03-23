use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use super::super::typings::user::user::User;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use crate::typings::user::user::UserGroup;
use crate::backend_api::response::user::login::Response as LoginResponse;
use crate::backend_api::response::user::common::Response as CommonResponse;
use crate::sqlite::sqlite::SQLite;
use rusqlite::{Transaction, Connection, params};
use uuid::Uuid;
use regex::Regex;
use std::fs;
use rocket::tokio::fs::create_dir;
use super::base_method::base::USER_IMG_PATH;

struct Password{
    pub password: String
}

#[post("/image_processing_website_api/login?<username>&<password>")]
fn login(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, username: String, password: String)-> Json<LoginResponse> {

    let mut _users = users.lock().unwrap();
    let user = _users.find_user_by_username(&username);
    if !user.is_none(){
        let login_response = LoginResponse::new(0, 1, 0, "".to_string());
        return Json(login_response);
    }

    let mut re = Regex::new("^[\\u0391-\\uFFE5A-Za-z0-9]+$").unwrap();
    if re.is_match(username.as_str()) == false{
        let login_response = LoginResponse::new(0, 1, 0, "".to_string());
        return Json(login_response);
    }
    re = Regex::new("^([a-zA-Z]*)(\\d*)([~!@#$%^&*()_+`\\-={}:\";'<>?,./]*).{6,20}$").unwrap();

    if re.is_match(password.as_str()) == false {
        let login_response = LoginResponse::new(0, 1, 0, "".to_string());
        return Json(login_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn: &Connection = _sqlite.getConn();
    let sql_lang = format!("SELECT USER_NAME, USER_PWD, AUTHORITY, TIME_STAMP FROM USERS WHERE USER_NAME='{}'", &username);

    let mut stmt_result = conn.prepare(sql_lang.as_str());
    if stmt_result.is_err(){
        println!("stmt is error");
        let login_response = LoginResponse::new(0, 1, 0, "".to_string());
        Json(login_response)
    }else {
        let mut stmt = stmt_result.unwrap();
        let users_result = stmt.query_map([], |row|{
            Ok(User {
                token: Uuid::new_v4().to_hyphenated().to_string(),
                username: row.get(0)?,
                password: row.get(1)?,
                authority: row.get(2)?,
                time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            })
        });
        if users_result.is_err(){
            println!("users_result is error");

            let mut login_response = LoginResponse::new(0, 1, 0, "".to_string());
            Json(login_response)
        }else {
            for user in users_result.unwrap() {
                if user.is_err(){
                    let login_response = LoginResponse::new(0, 1, 0, "".to_string());
                    return Json(login_response);
                }else {
                    let tmp_user = user.unwrap();
                    println!("{}", tmp_user);
                    _users.insert_user(&tmp_user);

                    let user_img_path_str = USER_IMG_PATH.to_string();
                    let tmp_user_img_path = Path::new(&user_img_path_str);
                    let user_img_path = tmp_user_img_path.join(username);
                    if !user_img_path.exists(){
                        let create_result = std::fs::create_dir(user_img_path);
                        if create_result.is_err(){
                            println!("create dir is error {:?}", create_result);
                        }
                    }

                    let login_response = LoginResponse::new(tmp_user.authority, 0, tmp_user.time_stamp, tmp_user.token);
                    return Json(login_response);
                }
            }
            let login_response = LoginResponse::new(0, 1, 0, "".to_string());
            Json(login_response)
        }
    }
}

#[post("/image_processing_website_api/register?<username>&<password>&<time_stamp>")]
fn register(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, username: String, password: String, time_stamp: u64) -> Json<CommonResponse>{

    let re = Regex::new("^([a-zA-Z]*)(\\d*)([~!@#$%^&*()_+`\\-={}:\";'<>?,./]*).{6,20}$").unwrap();
    if re.is_match(password.as_str()) == false {
        let resigter_response = CommonResponse::new(1);
        return Json(resigter_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let transcation: Transaction = _sqlite.getTransaction().unwrap();
    let execute_result =  transcation.execute("INSERT INTO USERS(USER_NAME, USER_PWD, AUTHORITY, TIME_STAMP) VALUES (?1, ?2, ?3, ?4)",
                                              params![username, password, 1, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()]);
    if execute_result.is_err() {
        let resigter_response = CommonResponse::new(1);
        return Json(resigter_response);
    }
    let insert_result = transcation.commit();
    if insert_result.is_err(){
        let resigter_response = CommonResponse::new(1);
        return Json(resigter_response);
    }

    let user_img_path_str = USER_IMG_PATH.to_string();
    let tmp_user_img_path = Path::new(&user_img_path_str);
    let user_img_path = tmp_user_img_path.join(username);

    if !user_img_path.exists(){
        let create_result = std::fs::create_dir(user_img_path);
        if create_result.is_err(){
            println!("create dir is error {:?}", create_result);
        }
    }
    let resigter_response = CommonResponse::new(0);
    Json(resigter_response)
}

#[post("/image_processing_website_api/resetPassword?<token>&<username>&<old_password>&<new_password>")]
fn resetPassword(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String, username: String, old_password: String, new_password: String) -> Json<CommonResponse> {

    let re = Regex::new("^([a-zA-Z]*)(\\d*)([~!@#$%^&*()_+`\\-={}:\";'<>?,./]*).{6,20}$").unwrap();
    if re.is_match(old_password.as_str()) == false {
        println!("old is error");
        let reset_password_response = CommonResponse::new(1);
        return Json(reset_password_response);
    }
    if re.is_match(new_password.as_str()) == false {
        println!("new is error");
        let reset_password_response = CommonResponse::new(1);
        return Json(reset_password_response);
    }
    if new_password == old_password {
        let reset_password_response = CommonResponse::new(1);
        return Json(reset_password_response);
    }
    if users.lock().unwrap().find_token(&token) == false {
        println!("reset_password_response");
        let reset_password_response = CommonResponse::new(1);
        return Json(reset_password_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn: &Connection = _sqlite.getConn();

    let select_sql = format!("SELECT USER_PWD FROM USERS WHERE USER_NAME='{}'", username);

    let stmt_result = conn.prepare(select_sql.as_str());
    if stmt_result.is_err() {
        println!("stmt is error {:?}", stmt_result);
        let reset_password_response = CommonResponse::new(1);
        return Json(reset_password_response);
    }
    let mut stmt = stmt_result.unwrap();

    let password_result = stmt.query_map([], |row| {
        let mut password = String::new();
        Ok(
            Password{
                password: row.get(0)?
            }
        )
    });
    if password_result.is_err(){
        println!("password_result is error");

        let reset_password_response = CommonResponse::new(1);
        return Json(reset_password_response);
    }

    let passwords = password_result.unwrap();
    for password in passwords {
        if password.is_err(){
            println!("password is error");

            let reset_password_response = CommonResponse::new(1);
            return Json(reset_password_response);
        }
        if old_password.eq(&password.unwrap().password) {
            let execute_result =  conn.execute("UPDATE USERS SET USER_PWD=?1 WHERE USER_NAME=?2", params![new_password, username]);
            if execute_result.is_err() {
                println!("execute is error {:?}", execute_result);
                let reset_password_response = CommonResponse::new(1);
                return Json(reset_password_response);
            }

            let reset_password_response = CommonResponse::new(0);
            return Json(reset_password_response);
        }
    }
    let reset_password_response = CommonResponse::new(1);
    return Json(reset_password_response);
}

#[post("/image_processing_website_api/checkManagerAuthority?<token>")]
fn checkManagerAuthority(users: &State<Mutex<UserGroup>>, token: String)-> Json<CommonResponse> {

    let mut users_result = users.lock().unwrap();
    let user_result = users_result.find_user_by_token(&token);
    if user_result.is_none(){
        let check_manager_authority_response = CommonResponse::new(1);
        return Json(check_manager_authority_response)
    }

    let resigter_response = CommonResponse::new(0);
    Json(resigter_response)
}

#[post("/image_processing_website_api/checkPassword?<token>&<username>&<password>")]
fn checkPassword(users: &State<Mutex<UserGroup>>, sqlite: &State<Mutex<SQLite>>, token: String, username: String, password: String) -> Json<CommonResponse> {
    let mut _users = users.lock().unwrap();
    if _users.find_token(&token) == false {
        let resigter_response = CommonResponse::new(1);
        return Json(resigter_response);
    }

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();
    let stmt_result = conn.prepare(format!("SELECT COUNT(*) FROM USERS WHERE USER_NAME='{}' AND USER_PWD='{}'", username, password).as_str());
    if stmt_result.is_err(){
        println!("stmt is error {:?}", stmt_result);
        let resigter_response = CommonResponse::new(1);
        return Json(resigter_response);
    }

    let mut stmt = stmt_result.unwrap();
    let count: i64 =  stmt.query_row(params![], |row| row.get(0)).unwrap();
    if count == 0 {
        println!("query_row is error");
        let resigter_response = CommonResponse::new(1);
        return Json(resigter_response);
    }


    let resigter_response = CommonResponse::new(0);
    Json(resigter_response)
}

pub fn get_routes() -> Vec<Route> {
    return routes![self::login, self::register, self::resetPassword, self::checkManagerAuthority, self::checkPassword]
}