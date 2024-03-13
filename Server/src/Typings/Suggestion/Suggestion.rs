use std::time::{SystemTime, UNIX_EPOCH};
use rocket::serde::{Deserialize, Serialize};
use rocket::time::macros::date;

#[derive(Clone, Deserialize, Serialize)]
struct Suggestion{
    pub suggestion_id: String,
    pub response: String,
    pub content: String,
    pub user_name: String,
    pub time_stamp: u64
}

impl Suggestion{
    pub fn new()->Suggestion {
        return Suggestion{
            suggestion_id: "".to_string(),
            response: "".to_string(),
            content: "".to_string(),
            user_name: "".to_string(),
            time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs(),
        }
    }
}