#![allow(dead_code)]
pub mod client;
pub mod errors;
pub mod structs;

extern crate regex;

#[allow(unused_imports)]
use structs::enrollments::{
    CreateFaceEnrollmentByUrlReturn, CreateFaceEnrollmentReturn, CreateVideoEnrollmentByUrlReturn,
    CreateVideoEnrollmentReturn, CreateVoiceEnrollmentByUrlReturn, CreateVoiceEnrollmentReturn,
    DeleteAllEnrollmentsReturn, GetAllFaceEnrollmentsReturn, GetAllVideoEnrollmentsReturn,
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

#[allow(unused_imports)]
use structs::subaccounts::{
    CreateSubAccountReturn,RegenerateSubAccountAPITokenReturn,DeleteSubAccountReturn,
};


use regex::Regex;
use std::fs::File;
use std::io::copy;

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

fn is_api_key(text: &str) -> bool {
    let re: Regex = Regex::new(r"^key_[a-zA-Z0-9]{32}$").unwrap();
    re.is_match(text)
}

fn download_file(link: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::get(link)?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let pwd = std::env::var("PWD").unwrap();
        let fname = format!("{}/{}", pwd, fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    copy(&mut response, &mut dest)?;
    Ok(())
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

        let mut x = crate::client::VoiceIt2::new(
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

        let result: crate::structs::users::CheckUserExistsReturn = match &x.check_user_exists(&user_id) {
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

        let result: crate::structs::groups::GetAllGroupsReturn = match &x.get_all_groups() {
                Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
                Err(err) => {
                    panic!("Panic error: {:?}", err);
                }
            };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::GetGroupReturn = match &x.get_group(&group_id) {
                Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
                Err(err) => {
                    panic!("Panic error: {:?}", err);
                }
            };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::CheckGroupExistsReturn = match &x.check_group_exists(&group_id) {
                Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
                Err(err) => {
                    panic!("Panic error: {:?}", err);
                }
            };

        assert_eq!(result.status, 200);
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

        let result: crate::structs::users::DeleteUserReturn = match &x.delete_user(&user_id) {
                Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
                Err(err) => {
                    panic!("Panic error: {:?}", err);
                }
            };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::DeleteGroupReturn = match &x.delete_group(&group_id) {
                Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
                Err(err) => {
                    panic!("Panic error: {:?}", err);
                }
            };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::phrases::GetPhrasesReturn = match &x.get_phrases("en-US") {
                Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
                Err(err) => {
                    panic!("Panic error: {:?}", err);
                }
            };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        &x.add_notification_url("https://voiceit.io");
        assert_eq!(
            &x.get_base_url(),
            "https://api.voiceit.io?notificationURL=https%3A%2F%2Fvoiceit.io"
        );
        &x.remove_notification_url();
        assert_eq!(&x.get_base_url(), "https://api.voiceit.io");
    }

    #[test]
    fn test_subaccounts(){
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

        let result: crate::structs::subaccounts::CreateSubAccountReturn = match &x.create_managed_sub_account("test", "rust", "", "", "") {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };
    
        assert_eq!(result.status, 201);
        let sub_account_managed_api_key = result.apiKey;
        assert!(crate::is_api_key(&sub_account_managed_api_key));
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::subaccounts::CreateSubAccountReturn = match &x.create_unmanaged_sub_account("test", "rust", "", "", "") {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };
    
        assert_eq!(result.status, 201);
        let sub_account_unmanaged_api_key = result.apiKey;
        assert!(crate::is_api_key(&sub_account_unmanaged_api_key));
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::subaccounts::SwitchSubAccountTypeReturn = match &x.switch_sub_account_type(&sub_account_unmanaged_api_key) {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };
    
        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::subaccounts::RegenerateSubAccountAPITokenReturn = match &x.regenerate_sub_account_api_token(&sub_account_managed_api_key) {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };
    
        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::subaccounts::DeleteSubAccountReturn = match &x.delete_subaccount(&sub_account_managed_api_key) {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };
    
        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::subaccounts::DeleteSubAccountReturn = match &x.delete_subaccount(&sub_account_unmanaged_api_key) {
            Ok(x) => serde_json::from_str(&x).expect(&x),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };
    
        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");
    }

    #[test]
    fn test_io() {
        println!("To be filled out later");
    }

    #[test]
    fn test_voice() {
        crate::download_file("https://drive.voiceit.io/files/enrollmentA1.wav").unwrap();
        crate::download_file("https://drive.voiceit.io/files/enrollmentA2.wav").unwrap();
        crate::download_file("https://drive.voiceit.io/files/enrollmentA3.wav").unwrap();
        crate::download_file("https://drive.voiceit.io/files/verificationA1.wav").unwrap();

        let pwd = std::env::var("PWD").unwrap();
        let x = crate::client::VoiceIt2::new(
            std::env::var("VIAPIKEY").unwrap(),
            std::env::var("VIAPITOKEN").unwrap(),
        );

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id1 = result.userId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id2 = result.userId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::CreateGroupReturn = match &x.create_group("example group") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let group_id = result.groupId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::AddUserToGroupReturn = match &x.add_user_to_group(&group_id, &user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::AddUserToGroupReturn = match &x.add_user_to_group(&group_id, &user_id2) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVoiceEnrollmentReturn = match &x.create_voice_enrollment(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/enrollmentA1.wav", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVoiceEnrollmentReturn = match &x.create_voice_enrollment(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/enrollmentA2.wav", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVoiceEnrollmentReturn = match &x.create_voice_enrollment(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/enrollmentA3.wav", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVoiceEnrollmentReturn = match &x.create_voice_enrollment_by_url(&user_id2, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/enrollmentC1.wav") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVoiceEnrollmentReturn = match &x.create_voice_enrollment_by_url(&user_id2, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/enrollmentC2.wav") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVoiceEnrollmentReturn = match &x.create_voice_enrollment_by_url(&user_id2, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/enrollmentC3.wav") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::GetAllVoiceEnrollmentsReturn = match &x.get_all_voice_enrollments(&user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::verification::VoiceVerificationReturn = match &x.voice_verification(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/verificationA1.wav", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::verification::VoiceVerificationReturn = match &x.voice_verification_by_url(&user_id1, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/enrollmentA4.wav") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::identification::VoiceIdentificationReturn = match &x.voice_identification(&group_id, "en-US", "never forget tomorrow is a new day", format!("{}/verificationA1.wav", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.userId, user_id1);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::identification::VoiceIdentificationReturn = match &x.voice_identification_by_url(&group_id, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/enrollmentA4.wav") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.userId, user_id1);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::DeleteAllEnrollmentsReturn = match &x.delete_all_enrollments(&user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::DeleteAllEnrollmentsReturn = match &x.delete_all_enrollments(&user_id2) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        &x.delete_user(&user_id1).unwrap();
        &x.delete_user(&user_id2).unwrap();
        &x.delete_group(&group_id).unwrap();

        std::fs::remove_file(format!("{}/{}", &pwd, "enrollmentA1.wav"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "enrollmentA2.wav"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "enrollmentA3.wav"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "verificationA1.wav"))
            .expect("Unable to delete file");
    }

    #[test]
    fn test_face() {
        crate::download_file("https://drive.voiceit.io/files/faceEnrollmentB1.mp4").unwrap();
        crate::download_file("https://drive.voiceit.io/files/faceEnrollmentB2.mp4").unwrap();
        crate::download_file("https://drive.voiceit.io/files/faceEnrollmentB3.mp4").unwrap();
        crate::download_file("https://drive.voiceit.io/files/faceVerificationB1.mp4").unwrap();

        let pwd = std::env::var("PWD").unwrap();

        let x = crate::client::VoiceIt2::new(
            std::env::var("VIAPIKEY").unwrap(),
            std::env::var("VIAPITOKEN").unwrap(),
        );

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id1 = result.userId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id2 = result.userId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::CreateGroupReturn = match &x.create_group("example group") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let group_id = result.groupId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::AddUserToGroupReturn = match &x.add_user_to_group(&group_id, &user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::AddUserToGroupReturn = match &x.add_user_to_group(&group_id, &user_id2) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateFaceEnrollmentReturn = match &x.create_face_enrollment(&user_id1, format!("{}/faceEnrollmentB1.mp4", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateFaceEnrollmentReturn = match &x.create_face_enrollment(&user_id1, format!("{}/faceEnrollmentB2.mp4", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateFaceEnrollmentReturn = match &x.create_face_enrollment(&user_id1, format!("{}/faceEnrollmentB3.mp4", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateFaceEnrollmentReturn = match &x.create_face_enrollment_by_url(&user_id2, "https://drive.voiceit.io/files/videoEnrollmentC1.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateFaceEnrollmentReturn = match &x.create_face_enrollment_by_url(&user_id2, "https://drive.voiceit.io/files/videoEnrollmentC2.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateFaceEnrollmentReturn = match &x.create_face_enrollment_by_url(&user_id2, "https://drive.voiceit.io/files/videoEnrollmentC3.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::GetAllFaceEnrollmentsReturn = match &x.get_all_face_enrollments(&user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::verification::FaceVerificationReturn = match &x.face_verification(&user_id1, format!("{}/faceVerificationB1.mp4", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::verification::FaceVerificationReturn = match &x.face_verification_by_url(&user_id1, "https://drive.voiceit.io/files/faceVerificationB1.mp4") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::identification::FaceIdentificationReturn = match &x.face_identification(&group_id, format!("{}/faceVerificationB1.mp4", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.userId, user_id1);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::identification::FaceIdentificationReturn = match &x.face_identification_by_url(&group_id, "https://drive.voiceit.io/files/faceVerificationB1.mp4") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.userId, user_id1);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::DeleteAllEnrollmentsReturn = match &x.delete_all_enrollments(&user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::DeleteAllEnrollmentsReturn = match &x.delete_all_enrollments(&user_id2) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        std::fs::remove_file(format!("{}/{}", &pwd, "faceEnrollmentB1.mp4"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "faceEnrollmentB2.mp4"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "faceEnrollmentB3.mp4"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "faceVerificationB1.mp4"))
            .expect("Unable to delete file");
    }

    #[test]
    fn test_video() {
        crate::download_file("https://drive.voiceit.io/files/videoEnrollmentB1.mov").unwrap();
        crate::download_file("https://drive.voiceit.io/files/videoEnrollmentB2.mov").unwrap();
        crate::download_file("https://drive.voiceit.io/files/videoEnrollmentB3.mov").unwrap();
        crate::download_file("https://drive.voiceit.io/files/videoVerificationB1.mov").unwrap();

        let pwd = std::env::var("PWD").unwrap();

        let x = crate::client::VoiceIt2::new(
            std::env::var("VIAPIKEY").unwrap(),
            std::env::var("VIAPITOKEN").unwrap(),
        );

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id1 = result.userId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::users::CreateUserReturn = match &x.create_user() {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let user_id2 = result.userId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::CreateGroupReturn = match &x.create_group("example group") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        let group_id = result.groupId;
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::AddUserToGroupReturn = match &x.add_user_to_group(&group_id, &user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::groups::AddUserToGroupReturn = match &x.add_user_to_group(&group_id, &user_id2) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVideoEnrollmentReturn = match &x.create_video_enrollment(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/videoEnrollmentB1.mov", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVideoEnrollmentReturn = match &x.create_video_enrollment(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/videoEnrollmentB2.mov", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVideoEnrollmentReturn = match &x.create_video_enrollment(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/videoEnrollmentB3.mov", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVideoEnrollmentReturn = match &x.create_video_enrollment_by_url(&user_id2, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/videoEnrollmentC1.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVideoEnrollmentReturn = match &x.create_video_enrollment_by_url(&user_id2, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/videoEnrollmentC2.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::CreateVideoEnrollmentReturn = match &x.create_video_enrollment_by_url(&user_id2, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/videoEnrollmentC3.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 201);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::GetAllVideoEnrollmentsReturn = match &x.get_all_video_enrollments(&user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::verification::VideoVerificationReturn = match &x.video_verification(&user_id1, "en-US", "never forget tomorrow is a new day", format!("{}/videoVerificationB1.mov", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::verification::VideoVerificationReturn = match &x.video_verification_by_url(&user_id1, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/videoVerificationB1.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::identification::VideoIdentificationReturn = match &x.video_identification(&group_id, "en-US", "never forget tomorrow is a new day", format!("{}/videoVerificationB1.mov", &pwd).as_str()) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.userId, user_id1);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::identification::VideoIdentificationReturn = match &x.video_identification_by_url(&group_id, "en-US", "never forget tomorrow is a new day", "https://drive.voiceit.io/files/videoVerificationB1.mov") {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.userId, user_id1);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::DeleteAllEnrollmentsReturn = match &x.delete_all_enrollments(&user_id1) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        let result: crate::structs::enrollments::DeleteAllEnrollmentsReturn = match &x.delete_all_enrollments(&user_id2) {
            Ok(x) => serde_json::from_str(&x).expect(format!("Unable to unmarshal JSON properly due to call failing and returning with missing values. server response: {}", &x).as_str()),
            Err(err) => {
                panic!("Panic error: {:?}", err);
            }
        };

        assert_eq!(result.status, 200);
        assert_eq!(result.responseCode, "SUCC");

        std::fs::remove_file(format!("{}/{}", &pwd, "videoEnrollmentB1.mov"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "videoEnrollmentB2.mov"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "videoEnrollmentB3.mov"))
            .expect("Unable to delete file");
        std::fs::remove_file(format!("{}/{}", &pwd, "videoVerificationB1.mov"))
            .expect("Unable to delete file");
    }
}
