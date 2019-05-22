#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub createdAt: u64,
    pub description: String,
    pub groupId: String,
    pub users: Vec<String>,
    pub userCount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllGroupsReturn {
    pub message: String,
    pub count: u64,
    pub status: u16,
    pub timeTaken: String,
    pub groups: Vec<Group>,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGroupReturn {
    pub message: String,
    pub description: String,
    pub groupId: String,
    pub createdAt: u64,
    pub users: Vec<String>,
    pub userCount: u64,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckGroupExistsReturn {
    pub message: String,
    pub exists: bool,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGroupReturn {
    pub message: String,
    pub description: String,
    pub groupId: String,
    pub status: u16,
    pub createdAt: u64,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddUserToGroupReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveUserFromGroupReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteGroupReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}
