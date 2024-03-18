use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use serde::Deserializer;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, FromForm, Debug)]
pub struct User {
    pub token: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new()->User
    {
        return User{
            token: Uuid::new_v4().to_hyphenated().to_string(),
            username: "".to_string(),
            password: "".to_string(),
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
    pub tokens: HashMap<String, u8>
}

impl UserGroup{
    pub fn new()->UserGroup
    {
        return UserGroup{
            users: BTreeMap::new(),
            tokens: HashMap::new()
        }
    }

    pub fn find_user(&mut self, user_name: &String)->Option<&User>
    {
        return self.users.get(user_name)
    }

    pub fn insert_user(&mut self, user: &User)
    {
        self.users.insert(user.username.clone(), user.clone());
        self.tokens.insert(user.token.clone(), 0);
    }

    pub fn erase_user(&mut self, user_name: &String)
    {
        self.tokens.remove(&self.users.get_mut(user_name).unwrap().token);
        self.users.remove(user_name);
    }

    pub fn change_user(&mut self, user_name:&String, password: &String)
    {
        self.users.get_mut(user_name).unwrap().password = password.clone();
    }

    pub fn find_token(&mut self, token: &String)->bool
    {
        return !self.tokens.get(token).is_none()
    }
}