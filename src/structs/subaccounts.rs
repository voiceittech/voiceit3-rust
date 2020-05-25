#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSubAccountReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub password: String,
    pub apiToken: String,
    pub apiKey: String,
    pub email: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegenerateSubAccountAPITokenReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub apiToken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteSubAccountReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwitchSubAccountTypeReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}
