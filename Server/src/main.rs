#[macro_use] extern crate rocket;

mod user;
use user::user as User;
use rocket::fairing::AdHoc;

// #[derive(Deserialize)]
// struct AppConfig {
//     id: Option<usize>,
//     port: u16,
// }

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
fn rocket() -> _ {
    // rocket::build().attach(AdHoc::config::<AppConfig>())
    rocket::build().mount("/", routes![index])

}
