use std::collections::BTreeMap;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new()->User
    {
        return User{
            username: "".to_string(),
            password: "".to_string(),
        }
    }

    pub fn set_username(&mut self, username: String)
    {
        self.username = username;
    }

    pub fn set_password(&mut self, password: String)
    {
        self.password = password;
    }

}

pub struct UserGroup{
    pub users: BTreeMap<String, User>,
}

impl UserGroup{
    pub fn new()->UserGroup
    {
        return UserGroup{
            users: BTreeMap::new(),
        }
    }

    pub fn find_user(&mut self, user_name: &String)->Option<&User>
    {
        return self.users.get(user_name)
    }

    pub fn insert_user(&mut self, user: &User)
    {
        self.users.insert(user.username.clone(), user.clone());
    }

    pub fn erase_user(&mut self, user_name: &String)
    {
        self.users.remove(user_name);
    }

    pub fn change_user(&mut self, user_name:&String, password: &String)
    {
        self.users.get_mut(user_name).unwrap().password = password.clone();
    }


}