use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserName{
    pub user_name: String
}

#[derive(Deserialize)]
pub struct GetUserList{
    pub user_name: Option<String>,
    pub from: Option<i64>,
    pub count: Option<i64>
}

#[derive(Deserialize)]
pub struct UserID{
    pub user_id: String
}

#[derive(Deserialize)]
pub struct UpdateUser{
    pub user_id: String,
    pub user_name: String
}