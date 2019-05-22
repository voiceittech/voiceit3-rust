extern crate reqwest;

#[derive(Debug)]
pub enum VoiceItError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

impl From<std::io::Error> for VoiceItError {
    fn from(err: std::io::Error) -> VoiceItError {
        VoiceItError::Io(err)
    }
}

impl From<reqwest::Error> for VoiceItError {
    fn from(err: reqwest::Error) -> VoiceItError {
        VoiceItError::Reqwest(err)
    }
}
