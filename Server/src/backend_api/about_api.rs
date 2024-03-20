use std::process::abort;
use std::sync::Mutex;
use super::super::typings::user::user::User;
use crate::backend_api::request;
use rocket::fairing::AdHoc;
use rocket::{Route, State};
use rocket::serde::json::Json;
use rusqlite::params;
use crate::backend_api::response::about::about::{OfficalUrl, Response as AboutResponse};
use crate::sqlite::sqlite::SQLite;

struct About{
    pub introduction: String,
    pub offical_url: String,
    pub download_url: String,
    pub recommended_article_url: String,
}

#[get("/image_processing_website_api/about?<token>&<target_content_title>")]
fn about(sqlite: &State<Mutex<SQLite>>, token: String, target_content_title: String)->Json<AboutResponse>{

    let mut _sqlite = sqlite.lock().unwrap();
    let conn = _sqlite.getConn();

    let stmt_result = conn.prepare(format!("SELECT INTRODUCTION, OFFICAL_URL, DOWLOAD_URL, RECOMMENDED_ARTICLE_URL From About WHERE TITLE='{}'", target_content_title).as_str());
    if stmt_result.is_err(){
        println!("stmt is error {:?}", stmt_result);
        let about_response = AboutResponse::new(vec![], "".to_string(), vec![], vec![], 1);
        return Json(about_response);
    }

    let mut stmt = stmt_result.unwrap();

    let about_result = stmt.query_map(params![], |row| {
        Ok(
            About{
                introduction: row.get(0)?,
                offical_url: row.get(1)?,
                download_url: row.get(2)?,
                recommended_article_url: row.get(3)?,
            }
        )
    });

    let abouts = about_result.unwrap();
    for about in abouts {
        if about.is_err(){
            println!("about is error");
            break;
        }
        let unwrap_about = about.unwrap();
        let about_response = AboutResponse::new(
                                                rocket::serde::json::from_str(&unwrap_about.download_url).unwrap(),
                                                unwrap_about.introduction,
                                                rocket::serde::json::from_str(&unwrap_about.offical_url).unwrap(),
                                                rocket::serde::json::from_str(&unwrap_about.recommended_article_url).unwrap(),
                                                0);
        return Json(about_response);
    }


    let about_response = AboutResponse::new(vec![], "".to_string(), vec![], vec![], 1);
    Json(about_response)
}

pub fn get_routes()-> Vec<Route> {
    return routes![self::about]
}