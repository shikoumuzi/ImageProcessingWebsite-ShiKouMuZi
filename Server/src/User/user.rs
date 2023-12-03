use std::collections::BTreeMap;

pub struct User {
    pub username: String,
    pub password: String,
    pub userid: String
}

impl User {
    pub fn new()->User
    {
        return User{
            username: "".to_string(),
            password: "".to_string(),
            userid: "".to_string(),
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

    pub fn set_userid(&mut self, userid: String)
    {
        self.userid = userid;
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

    pub fn find_user(&mut self, user_id: &String)->Option<&mut User>
    {
        return self.users.get_mut(user_id)
    }



}