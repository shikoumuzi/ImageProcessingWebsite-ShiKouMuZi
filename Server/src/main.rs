#[macro_use] extern crate rocket;

mod Typings;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use Typings::User::user as User;
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use crate::Typings::User::user::UserGroup;

// file server 如果需要绑定vue文件的话需要index和files函数配合 并且要记得介入到routes当中

#[get("/")]
async fn index() -> Option<NamedFile>{
    NamedFile::open("F:/University/WorkAndReport/GraduationProject/ImageProcessingWebsite/Web/imageprocessingwebsite/dist/index.html").await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("F:/University/WorkAndReport/GraduationProject/ImageProcessingWebsite/Web/imageprocessingwebsite/dist/").join(file)).await.ok()
}
 

#[launch]
fn rocket() -> _ {
    let users = Mutex::new( UserGroup::new() );
    // rocket::build().attach(AdHoc::config::<AppConfig>())
    rocket::build().
        manage(users).
        mount("/", routes![index, files])

}
