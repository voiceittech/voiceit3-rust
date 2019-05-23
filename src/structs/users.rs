#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub createdAt: u64,
    pub userId: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllUsersReturn {
    pub message: String,
    pub count: u64,
    pub status: u16,
    pub timeTaken: String,
    pub users: Vec<User>,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub userId: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckUserExistsReturn {
    pub message: String,
    pub exists: bool,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGroupsForUserReturn {
    pub message: String,
    pub groups: Vec<String>,
    pub count: u64,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserTokenReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub userToken: String,
    pub createdAt: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpireUserTokensReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}
