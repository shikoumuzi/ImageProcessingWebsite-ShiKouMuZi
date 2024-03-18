use rocket::http::StatusClass::Success;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::outcome::IntoOutcome;
use rocket::data::{Data, FromData, Outcome};
use crate::typings::user::user::User as User;

// #[rocket::async_trait]
// impl FromData<'_> for User {
//     type Error = ();
//
//     async fn from_data(req: &'_ Request<'_>, data: Data<'_>) -> Outcome<'_, Self> {
//
//         let data_limit = data.open()
//     }
// }