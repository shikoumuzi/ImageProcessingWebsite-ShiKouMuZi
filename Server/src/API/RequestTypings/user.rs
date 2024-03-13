use rocket::Request;
use rocket::request::FromRequest;
use rocket::outcome::IntoOutcome;
use crate::Typings::User;


impl FromRequest for User {
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let mut contents = String::new();

        if let Err(e) = data.open().take(256).read_to_string(&mut contents) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(Store { contents })
    }
}