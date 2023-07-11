extern crate base64;
extern crate hyper;
extern crate reqwest;
extern crate url;

pub use crate::errors::VoiceItError;
use reqwest::blocking::multipart;
pub use reqwest::blocking::Client;
pub use std::io::Read;
use url::form_urlencoded;

static PLATFORM_ID: &'static str = "49";
pub static PLATFORM_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct VoiceIt2 {
    api_key: String,
    api_token: String,
    base_url: String,
    notification_url_parameter: String,
}

impl VoiceIt2 {
    pub fn new(api_key: String, api_token: String) -> VoiceIt2 {
        VoiceIt2 {
            api_key: api_key,
            api_token: api_token,
            notification_url_parameter: String::from(""),
            base_url: String::from("https://api.voiceit.io"),
        }
    }

    pub fn new_custom(api_key: String, api_token: String, custom_url: String) -> VoiceIt2 {
        VoiceIt2 {
            api_key: api_key,
            api_token: api_token,
            notification_url_parameter: String::from(""),
            base_url: custom_url,
        }
    }

    // USERS

    pub fn get_base_url(&self) -> String {
        return format!("{}{}", self.base_url, self.notification_url_parameter);
    }

    pub fn add_notification_url(&mut self, url: &str) {
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("notificationURL", url)
            .finish();

        self.notification_url_parameter = format!("?{}", encoded);
    }

    pub fn remove_notification_url(&mut self) {
        self.notification_url_parameter = String::from("");
    }

    pub fn get_all_users(&self) -> Result<String, VoiceItError> {
        let url = format!("{}/users{}", self.base_url, self.notification_url_parameter);

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_user(&self) -> Result<String, VoiceItError> {
        let url = format!("{}/users{}", self.base_url, self.notification_url_parameter);

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn check_user_exists(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}{}",
            self.base_url, user_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn delete_user(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}{}",
            self.base_url, user_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .delete(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn get_groups_for_user(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}/groups{}",
            self.base_url, user_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_user_token(
        &self,
        user_id: &str,
        expiration_time_seconds: u64,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}/token?timeOut={}",
            self.base_url,
            user_id,
            expiration_time_seconds.to_string(),
        );

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn expire_user_tokens(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}/expireTokens{}",
            self.base_url, user_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    // SUBACCOUNTS
    pub fn create_managed_sub_account(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
        content_language: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/subaccount/managed{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("firstName", String::from(first_name))
            .text("lastName", String::from(last_name))
            .text("email", String::from(email))
            .text("password", String::from(password))
            .text("contentLanguage", String::from(content_language));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .multipart(form)
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_unmanaged_sub_account(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
        content_language: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/subaccount/unmanaged{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("firstName", String::from(first_name))
            .text("lastName", String::from(last_name))
            .text("email", String::from(email))
            .text("password", String::from(password))
            .text("contentLanguage", String::from(content_language));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .multipart(form)
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn switch_sub_account_type(
        &self,
        sub_account_api_key: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/subaccount/{}/switchType{}",
            self.base_url,
            String::from(sub_account_api_key),
            self.notification_url_parameter
        );

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn regenerate_sub_account_api_token(
        &self,
        sub_account_api_key: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/subaccount/{}{}",
            self.base_url,
            String::from(sub_account_api_key),
            self.notification_url_parameter
        );

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn delete_subaccount(&self, sub_account_api_key: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/subaccount/{}{}",
            self.base_url,
            String::from(sub_account_api_key),
            self.notification_url_parameter
        );

        let mut response = Client::new()
            .delete(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    // GROUPS

    pub fn create_group(&self, description: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new().text("description", String::from(description));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn get_all_groups(&self) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups{}",
            self.base_url, self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn get_group(&self, group_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups/{}{}",
            self.base_url,
            String::from(group_id),
            self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn check_group_exists(&self, group_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups/{}/exists{}",
            self.base_url,
            String::from(group_id),
            self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn add_user_to_group(&self, group_id: &str, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups/addUser{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("groupId", String::from(group_id));

        let mut response = Client::new()
            .put(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .multipart(form)
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn remove_user_from_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups/removeUser{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("groupId", String::from(group_id));

        let mut response = Client::new()
            .put(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .multipart(form)
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn delete_group(&self, group_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups/{}{}",
            self.base_url, group_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .delete(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    // PHRASES

    pub fn get_phrases(&self, content_language: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/phrases/{}{}",
            self.base_url, content_language, self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn delete_all_enrollments(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/{}/all{}",
            self.base_url,
            String::from(user_id),
            self.notification_url_parameter
        );

        let mut response = Client::new()
            .delete(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn get_all_voice_enrollments(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/voice/{}{}",
            self.base_url, user_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_voice_enrollment(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        recording_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/voice{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .file("recording", String::from(recording_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_voice_enrollment_by_url(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        recording_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/voice/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .text("fileUrl", String::from(recording_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn voice_verification(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        recording_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/verification/voice{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .file("recording", String::from(recording_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn voice_verification_by_url(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        recording_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/verification/voice/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .text("fileUrl", String::from(recording_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn voice_identification(
        &self,
        group_id: &str,
        content_language: &str,
        phrase: &str,
        recording_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/identification/voice{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("groupId", String::from(group_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .file("recording", String::from(recording_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn voice_identification_by_url(
        &self,
        group_id: &str,
        content_language: &str,
        phrase: &str,
        recording_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/identification/voice/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("groupId", String::from(group_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .text("fileUrl", String::from(recording_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn get_all_face_enrollments(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/face/{}{}",
            self.base_url, user_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_face_enrollment(
        &self,
        user_id: &str,
        video_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/face{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .file("video", String::from(video_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_face_enrollment_by_url(
        &self,
        user_id: &str,
        video_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/face/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("fileUrl", String::from(video_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn face_verification(
        &self,
        user_id: &str,
        video_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/verification/face{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .file("video", String::from(video_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn face_verification_by_url(
        &self,
        user_id: &str,
        video_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/verification/face/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("fileUrl", String::from(video_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn face_identification(
        &self,
        group_id: &str,
        video_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/identification/face{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("groupId", String::from(group_id))
            .file("video", String::from(video_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn face_identification_by_url(
        &self,
        group_id: &str,
        video_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/identification/face/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("groupId", String::from(group_id))
            .text("fileUrl", String::from(video_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn get_all_video_enrollments(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/video/{}{}",
            self.base_url, user_id, self.notification_url_parameter
        );

        let mut response = Client::new()
            .get(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_video_enrollment(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        video_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/video{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .file("video", String::from(video_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn create_video_enrollment_by_url(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        video_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/enrollments/video/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .text("fileUrl", String::from(video_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn video_verification(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        video_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/verification/video{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .file("video", String::from(video_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn video_verification_by_url(
        &self,
        user_id: &str,
        content_language: &str,
        phrase: &str,
        video_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/verification/video/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("userId", String::from(user_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .text("fileUrl", String::from(video_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn video_identification(
        &self,
        group_id: &str,
        content_language: &str,
        phrase: &str,
        video_path: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/identification/video{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("groupId", String::from(group_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .file("video", String::from(video_path))
            .unwrap();

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }

    pub fn video_identification_by_url(
        &self,
        group_id: &str,
        content_language: &str,
        phrase: &str,
        video_url: &str,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/identification/video/byUrl{}",
            self.base_url, self.notification_url_parameter
        );

        let form = multipart::Form::new()
            .text("groupId", String::from(group_id))
            .text("contentLanguage", String::from(content_language))
            .text("phrase", String::from(phrase))
            .text("fileUrl", String::from(video_url));

        let mut response = Client::new()
            .post(&url)
            .header("platformId", PLATFORM_ID)
            .header("platformVersion", PLATFORM_VERSION)
            .multipart(form)
            .basic_auth(self.api_key.clone(), Some(self.api_token.clone()))
            .send()?;

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        Ok(body)
    }
}
