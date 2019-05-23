extern crate base64;
extern crate hyper;
extern crate reqwest;

pub use crate::errors::VoiceItError;
use reqwest::multipart;
pub use reqwest::Client;
pub use std::io::Read;

static PLATFORM_ID: &'static str = "49";
pub static PLATFORM_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static BASE_URL: &'static str = "https://api.voiceit.io";

pub struct VoiceIt2 {
    api_key: String,
    api_token: String,
    notification_url_parameter: String,
}

impl VoiceIt2 {
    pub fn new(api_key: String, api_token: String) -> VoiceIt2 {
        VoiceIt2 {
            api_key: api_key,
            api_token: api_token,
            notification_url_parameter: String::from(""),
        }
    }

    // USERS

    pub fn get_all_users(&self) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users{}",
            String::from(BASE_URL),
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

    pub fn create_user(&self) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users{}",
            String::from(BASE_URL),
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

    pub fn check_if_user_exists(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}{}",
            String::from(BASE_URL),
            user_id,
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

    pub fn delete_user(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}{}",
            String::from(BASE_URL),
            user_id,
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

    pub fn get_groups_for_user(&self, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}/groups{}",
            String::from(BASE_URL),
            user_id,
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

    pub fn create_user_token(
        &self,
        user_id: &str,
        expiration_time_seconds: u64,
    ) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/users/{}/token?timeOut={}",
            String::from(BASE_URL),
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
            String::from(BASE_URL),
            user_id,
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

    // GROUPS

    pub fn create_group(&self, description: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups{}",
            String::from(BASE_URL),
            self.notification_url_parameter
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

    pub fn add_user_to_group(&self, group_id: &str, user_id: &str) -> Result<String, VoiceItError> {
        let url = format!(
            "{}/groups/addUser{}",
            String::from(BASE_URL),
            self.notification_url_parameter
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
            String::from(BASE_URL),
            self.notification_url_parameter
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
}
