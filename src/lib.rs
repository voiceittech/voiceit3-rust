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
    let re: Regex = Regex::new(r"^usr_[a-zA-Z0-9]{32}$").unwrap();
    re.is_match(text)
}

fn is_group_id(text: &str) -> bool {
    let re: Regex = Regex::new(r"^grp_[a-zA-Z0-9]{32}$").unwrap();
    re.is_match(text)
}

fn is_user_token(text: &str) -> bool {
    let re: Regex = Regex::new(r"^utk_[a-zA-Z0-9]{32}_[0-9]*$").unwrap();
    re.is_match(text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basics() {
        let env = std::env::var("BOXFUSE_ENV").unwrap();
        if env == "voiceittest" {
            let home_dir = std::env::var("HOME").unwrap();
            std::fs::write(
                format!("{}/platformVersion", home_dir),
                crate::client::PLATFORM_VERSION,
            )
            .expect("Unable to write platformVersion file");
        }

        let x = crate::client::VoiceIt2::new(
            std::env::var("VIAPIKEY").unwrap(),
            std::env::var("VIAPITOKEN").unwrap(),
        );

        let result: crate::structs::users::GetAllUsersReturn = match &x.get_all_users() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id = result.userId;
        assert!(crate::is_user_id(&user_id));
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::CheckUserExistsReturn = match &x.check_if_user_exists(&user_id) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.exists, true);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::CreateGroupReturn = match &x.create_group("sample group") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let group_id = result.groupId;
        assert!(crate::is_group_id(&group_id));
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::AddUserToGroupReturn = match &x.add_user_to_group(&group_id, &user_id) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::GetGroupsForUserReturn = match &x.get_groups_for_user(&user_id) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");
        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.groups[0], group_id);

        let result: crate::structs::groups::RemoveUserFromGroupReturn = match &x.remove_user_from_group(&group_id, &user_id) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::CreateUserTokenReturn = match &x.create_user_token(&user_id, 100000) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_token = result.userToken;
        assert!(crate::is_user_token(&user_token));
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::ExpireUserTokensReturn = match &x.expire_user_tokens(&user_id) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");
    }
}
