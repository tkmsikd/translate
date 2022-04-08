use curl::easy::{Easy2, Handler, WriteError};
use serde::{Serialize, Deserialize};
use crate::handler::service_handler::ServiceData;
use std::fmt;
use bincode;
pub struct Collector(pub Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum TargetLanguage {
    JP,
    EN,
    Undefined
}

impl fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetLanguage::JP => write!(f, "JP"),
            TargetLanguage::EN => write!(f, "EN"),
            TargetLanguage::Undefined => write!(f, "undefined")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HttpRequest{
    pub service_data: ServiceData,
    pub text: String,
    pub target_lang: TargetLanguage,
}

impl HttpRequest {
    pub fn generate_query(&self)  -> String{
        let url = &self.service_data.url;
        let auth_key = &self.service_data.authentication_key;
        let text = &self.text;
        let target_lang = self.target_lang.to_string();

        let full = format!("{}?auth_key={}?text={}?target_lang={}", url, auth_key, text, target_lang);
        full.to_string()
    }

    pub fn encode_query<T: Serialize>(&self, target: &T) -> Vec<u8> {
        bincode::serialize(&target).unwrap()
    }
}
