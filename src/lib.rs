#![allow(dead_code)]
pub mod client;
pub mod errors;
pub mod structs;

extern crate regex;

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

use regex::Regex;

fn is_user_id(text: &str) -> bool {
    let re: Regex = Regex::new(r"usr_[a-zA-Z0-9{32}]").unwrap();
    re.is_match(text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basics() {
        let x = crate::client::VoiceIt2::new(
            std::env::var("VIAPIKEY").unwrap(),
            std::env::var("VIAPITOKEN").unwrap(),
        );

        let result: crate::structs::users::GetAllUsers = match &x.get_all_users() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::CreateUser = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id = result.userId;
        assert!(crate::is_user_id(&user_id));
        assert_eq!(result.responseCode, "SUCC");
    }
}
