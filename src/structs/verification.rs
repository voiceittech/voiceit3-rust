#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceVerificationReturn {
    pub message: String,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceVerificationByUrlReturn {
    pub message: String,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaceVerificationReturn {
    pub message: String,
    pub status: u16,
    pub faceConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaceVerificationByUrlReturn {
    pub message: String,
    pub status: u16,
    pub faceConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoVerificationReturn {
    pub message: String,
    pub status: u16,
    pub voiceConfidence: f32,
    pub faceConfidence: f32,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoVerificationByUrlReturn {
    pub message: String,
    pub status: u16,
    pub voiceConfidence: f32,
    pub faceConfidence: f32,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}
