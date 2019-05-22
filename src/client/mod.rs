extern crate base64;
extern crate hyper;
extern crate reqwest;

pub use crate::errors::VoiceItError;
pub use reqwest::Client;
pub use std::io::Read;

static PLATFORM_ID: &'static str = "49";
static PLATFORM_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static BASE_URL: &'static str = "https://api.voiceit.io";

pub struct VoiceIt2 {
    api_key: String,
    api_token: String,
}

impl VoiceIt2 {
    pub fn new(api_key: String, api_token: String) -> VoiceIt2 {
        VoiceIt2 {
            api_key: api_key,
            api_token: api_token,
        }
    }

    // USERS

    pub fn get_all_users(&self) -> Result<String, VoiceItError> {
        let url = format!("{}/{}", String::from(BASE_URL), "users");

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
        let url = format!("{}/{}", String::from(BASE_URL), "users");

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
            "{}/{}/{}",
            String::from(BASE_URL),
            String::from("users"),
            user_id
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
            "{}/{}/{}",
            String::from(BASE_URL),
            String::from("users"),
            user_id
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
            "{}/{}/{}/{}",
            String::from(BASE_URL),
            String::from("users"),
            user_id,
            String::from("groups")
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
}
