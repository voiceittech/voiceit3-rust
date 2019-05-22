#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Phrase {
    pub text: String,
    pub contentLanguage: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPhrasesReturn {
    pub message: String,
    pub count: u64,
    pub status: u16,
    pub timeTaken: String,
    pub phrases: Vec<Phrase>,
    pub responseCode: String,
}
