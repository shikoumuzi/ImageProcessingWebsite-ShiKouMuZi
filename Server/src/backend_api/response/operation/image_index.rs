use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "status")]
    pub status: u8,
    #[serde(rename="mat_index")]
    pub mat_index: i32,
}
impl Response {
    pub fn new(status: u8, mat_index: i32) -> Self {
        Self { status, mat_index}
    }
}