// # 用户数据模型
use serde::{Serialize, Deserialize};
// use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]

// 注册用户
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub age: Option<String>,
    pub name: Option<String>,
    pub sex: Option<i32>,
    pub address: Option<String>,
    pub state: Option<i8>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub user_height: Option<String>,
    pub user_weight: Option<String>,
    pub disease: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// 用户信息
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub age: Option<String>,
    pub name: Option<String>,
    pub sex: Option<i32>,
    pub address: Option<String>,
    pub state: Option<i8>,
    pub phone: Option<String>,
    pub avatar: Option<String>,

    #[serde(rename = "userHeight")]
    pub user_height: Option<String>,

    #[serde(rename = "userWeight")]
    pub user_weight: Option<String>,
    pub disease: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub code: i32,
    pub msg: &'static str,
}