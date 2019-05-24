#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceEnrollment {
    pub createdAt: u64,
    pub contentLanguage: String,
    pub voiceEnrollmentId: u64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllVoiceEnrollmentsReturn {
    pub message: String,
    pub count: u64,
    pub status: u16,
    pub timeTaken: String,
    pub voiceEnrollments: Vec<VoiceEnrollment>,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaceEnrollment {
    pub createdAt: u64,
    pub faceEnrollmentId: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllFaceEnrollmentsReturn {
    pub message: String,
    pub count: u64,
    pub status: u16,
    pub timeTaken: String,
    pub faceEnrollments: Vec<FaceEnrollment>,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoEnrollment {
    pub createdAt: u64,
    pub contentLanguage: String,
    pub videoEnrollmentId: u64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllVideoEnrollmentsReturn {
    pub message: String,
    pub count: u64,
    pub status: u16,
    pub timeTaken: String,
    pub videoEnrollments: Vec<VideoEnrollment>,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateVoiceEnrollmentReturn {
    pub message: String,
    pub contentLanguage: String,
    pub id: u64,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub createdAt: u64,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateVoiceEnrollmentByUrlReturn {
    pub message: String,
    pub contentLanguage: String,
    pub id: u64,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub createdAt: u64,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFaceEnrollmentReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub faceEnrollmentId: u64,
    pub createdAt: u64,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFaceEnrollmentByUrlReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub faceEnrollmentId: u64,
    pub createdAt: u64,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateVideoEnrollmentReturn {
    pub message: String,
    pub contentLanguage: String,
    pub id: u64,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub createdAt: u64,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateVideoEnrollmentByUrlReturn {
    pub message: String,
    pub contentLanguage: String,
    pub id: u64,
    pub status: u16,
    pub text: String,
    pub textConfidence: f32,
    pub createdAt: u64,
    pub timeTaken: String,
    pub responseCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAllEnrollmentsReturn {
    pub message: String,
    pub status: u16,
    pub timeTaken: String,
    pub responseCode: String,
}
