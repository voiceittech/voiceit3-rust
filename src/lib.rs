pub mod client;
pub mod errors;
pub mod structs;

use structs::enrollments::{
    CreateFaceEnrollmentByUrlReturn, CreateFaceEnrollmentReturn, CreateVideoEnrollmentByUrlReturn,
    CreateVideoEnrollmentReturn, CreateVoiceEnrollmentByUrlReturn, CreateVoiceEnrollmentReturn,
    DeleteAllEnrollmentsReturn, DeleteAllFaceEnrollmentsReturn, DeleteAllVideoEnrollmentsReturn,
    DeleteAllVoiceEnrollmentsReturn, DeleteFaceEnrollmentReturn, DeleteVideoEnrollmentReturn,
    DeleteVoiceEnrollmentReturn, GetAllFaceEnrollmentsReturn, GetAllVideoEnrollmentsReturn,
    GetAllVoiceEnrollmentsReturn,
};

use structs::groups::{
    AddUserToGroupReturn, CheckGroupExistsReturn, CreateGroupReturn, DeleteGroupReturn,
    GetAllGroupsReturn, GetGroupReturn, RemoveUserFromGroupReturn,
};

use structs::identification::{
    FaceIdentificationByUrlReturn, FaceIdentificationReturn, VideoIdentificationByUrlReturn,
    VideoIdentificationReturn, VoiceIdentificationByUrlReturn, VoiceIdentificationReturn,
};

use structs::phrases::GetPhrasesReturn;

use structs::users::{
    CheckUserExistsReturn, CreateUserReturn, CreateUserTokenReturn, DeleteUserReturn,
    ExpireUserTokensReturn, GetAllUsersReturn, GetGroupsForUserReturn,
};

use structs::verification::{
    FaceVerificationByUrlReturn, FaceVerificationReturn, VideoVerificationByUrlReturn,
    VideoVerificationReturn, VoiceVerificationByUrlReturn, VoiceVerificationReturn,
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_basics() {
        let x = client::VoiceIt2::new(std::env("VIAPIKEY"), std::env("VIAPIKEY"));
        let result: structs::users::CreateUser = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        // let userId = result.userId;
        assert_eq!(result.responseCode, "SUCC");
    }
}
