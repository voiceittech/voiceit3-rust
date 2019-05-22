#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceIdentificationReturn {
    pub message: String,
    pub userId: String,
    pub groupId: String,
    pub confidence: f32,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceIdentificationByUrlReturn {
    pub message: String,
    pub userId: String,
    pub groupId: String,
    pub confidence: f32,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaceIdentificationReturn {
    pub message: String,
    pub userId: String,
    pub groupId: String,
    pub status: u16,
    pub faceConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaceIdentificationByUrlReturn {
    pub message: String,
    pub userId: String,
    pub groupId: String,
    pub status: u16,
    pub faceConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoIdentificationReturn {
    pub message: String,
    pub userId: String,
    pub groupId: String,
    pub status: u16,
    pub voiceConfidence: f32,
    pub faceConfidence: f32,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoIdentificationByUrlReturn {
    pub message: String,
    pub userId: String,
    pub groupId: String,
    pub status: u16,
    pub voiceConfidence: f32,
    pub faceConfidence: f32,
    pub text: String,
    pub textConfidence: f32,
    pub timeTaken: String,
    pub responseCode: String,
}
