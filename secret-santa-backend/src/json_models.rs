use serde::Deserialize;

#[derive(Deserialize)]
pub struct Username {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserGroupName {
    pub username: String,
    pub group_name: String,
}

#[derive(Deserialize)]
pub struct NewAdminUserGroupName {
    pub username: String,
    pub group_name: String,
    pub new_admin: String,
}
