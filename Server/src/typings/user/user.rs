use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use serde::Deserializer;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use std::option::Option;

#[derive(Clone, Deserialize, Serialize, FromForm, Debug)]
pub struct User {
    pub token: String,
    pub username: String,
    pub password: String,
    pub authority: u8,
    pub time_stamp: u64,
}

impl User {
    pub fn new()->User
    {
        return User{
            token: Uuid::new_v4().to_hyphenated().to_string(),
            username: "".to_string(),
            password: "".to_string(),
            authority: 0,
            time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }

    pub fn set_token(&mut self, token: &String) { self.token = token.clone()}

    pub fn set_username(&mut self, username: &String)
    {
        self.username = username.clone();
    }

    pub fn set_password(&mut self, password: &String)
    {
        self.password = password.clone();
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "username: {}; password: {}; token: {}", self.username, self.password, self.token)
    }
}


pub struct UserGroup{
    pub users: BTreeMap<String, User>,
    pub tokens: HashMap<String, User>
}

impl UserGroup{
    pub fn new()->UserGroup
    {
        return UserGroup{
            users: BTreeMap::new(),
            tokens: HashMap::new()
        }
    }

    pub fn find_user_by_username(&mut self, user_name: &String) ->Option<&User>
    {
        return self.users.get(user_name)
    }

    pub fn find_user_by_token(&mut self, token: &String)-> Option<&User>
    {
        let user = self.tokens.get(token);
        if user.is_none(){
            return None
        }
        return Option::from(user);
    }

    pub fn insert_user(& mut self, user: &User)
    {
        let token = user.token.clone();
        self.users.insert(user.username.clone(), user.clone());
        self.tokens.insert(user.token.clone(), user.clone());
    }

    pub fn erase_user(&mut self, user_name: &String)
    {
        self.tokens.remove(&self.users.get_mut(user_name).unwrap().token);
        self.users.remove(user_name);
    }

    pub fn change_user(&mut self, user_name:&String, password: &String)
    {
        self.users.get_mut(user_name).unwrap().password = password.clone();
        self.tokens.get_mut(&self.users.get(user_name).unwrap().token).unwrap().password = password.clone()
    }

    pub fn find_token(&mut self, token: &String)->bool
    {
        return !self.tokens.get(token).is_none()
    }

}