#[macro_use] extern crate rocket;

mod typings;
mod backend_api;
mod base_macro;


use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;

use typings::user::user as User;
use typings::user::user::UserGroup;

use backend_api::user_api;
use backend_api::about_api;
use backend_api::suggestion_api;
use backend_api::manager_api;


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
    println!("{}",url!("dasdads"));
    let users = Mutex::new( UserGroup::new() );

    let mut routes = routes![index, files];

    routes.append(&mut user_api::get_routes());
    routes.append(&mut about_api::get_routes());
    routes.append(&mut suggestion_api::get_routes());
    routes.append(&mut manager_api::get_routes());

    println!("{:?}", routes);
    // rocket::build().attach(AdHoc::config::<AppConfig>())
    rocket::build().
        manage(users).
        mount("/", routes)

}
