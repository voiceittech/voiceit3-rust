#![allow(dead_code)]
pub mod client;
pub mod errors;
pub mod structs;

#[allow(unused_imports)]
use structs::enrollments::{
    CreateFaceEnrollmentByUrlReturn, CreateFaceEnrollmentReturn, CreateVideoEnrollmentByUrlReturn,
    CreateVideoEnrollmentReturn, CreateVoiceEnrollmentByUrlReturn, CreateVoiceEnrollmentReturn,
    DeleteAllEnrollmentsReturn, DeleteAllFaceEnrollmentsReturn, DeleteAllVideoEnrollmentsReturn,
    DeleteAllVoiceEnrollmentsReturn, DeleteFaceEnrollmentReturn, DeleteVideoEnrollmentReturn,
    DeleteVoiceEnrollmentReturn, GetAllFaceEnrollmentsReturn, GetAllVideoEnrollmentsReturn,
    GetAllVoiceEnrollmentsReturn,
};

#[allow(unused_imports)]
use structs::groups::{
    AddUserToGroupReturn, CheckGroupExistsReturn, CreateGroupReturn, DeleteGroupReturn,
    GetAllGroupsReturn, GetGroupReturn, RemoveUserFromGroupReturn,
};

#[allow(unused_imports)]
use structs::identification::{
    FaceIdentificationByUrlReturn, FaceIdentificationReturn, VideoIdentificationByUrlReturn,
    VideoIdentificationReturn, VoiceIdentificationByUrlReturn, VoiceIdentificationReturn,
};

#[allow(unused_imports)]
use structs::phrases::GetPhrasesReturn;

#[allow(unused_imports)]
use structs::users::{
    CheckUserExistsReturn, CreateUserReturn, CreateUserTokenReturn, DeleteUserReturn,
    ExpireUserTokensReturn, GetAllUsersReturn, GetGroupsForUserReturn,
};

#[allow(unused_imports)]
use structs::verification::{
    FaceVerificationByUrlReturn, FaceVerificationReturn, VideoVerificationByUrlReturn,
    VideoVerificationReturn, VoiceVerificationByUrlReturn, VoiceVerificationReturn,
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_basics() {
        let x = crate::client::VoiceIt2::new(std::env::var("VIAPIKEY").unwrap(), std::env::var("VIAPITOKEN").unwrap());

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id = result.userId;
        assert_eq!(result.responseCode, "SUCC");
    }
}
