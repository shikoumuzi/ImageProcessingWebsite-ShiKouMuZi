use rocket::data::{FromData, Outcome};
use rocket::fs::TempFile;
use rocket::{Data, Request};

#[derive(FromForm)]
pub struct Image<'f>{
    pub token: String,
    pub file_name: String,
    pub file: TempFile<'f>
}
