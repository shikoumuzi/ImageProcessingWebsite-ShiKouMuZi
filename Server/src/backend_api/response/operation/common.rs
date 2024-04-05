use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "status")]
    pub status: u8,
}
impl Response {
    pub fn new(status: u8) -> Self {
        Self { status }
    }
}