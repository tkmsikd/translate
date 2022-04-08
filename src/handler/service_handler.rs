use serde::de::value::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum ServiceType {
    Deepl,
    GoogleTranslate,
}

impl ServiceType {
    pub fn load_service_type(service: &str) -> Self {
        match service {
            "google" => Self::GoogleTranslate,
            "deepl" | _ => Self::Deepl,
        }
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceData {
    pub url: String,
    pub authentication_key: String,
}

impl ServiceData {
    pub fn new(&self, service_type: ServiceType) -> Self {
        let (url, auth_key) = self.load_service_data(service_type).unwrap();

        Self {
            url: url,
            authentication_key: auth_key,
        }
    }

    fn load_service_data(&self, service_type: ServiceType) -> Result<(String, String), Box<Error>> {
        let credential_path = match service_type {
            ServiceType::Deepl => format!("./auth_files/deepl_auth_info.json"),
            ServiceType::GoogleTranslate => format!("./auth_files/gtranslate_auth_info.json"),
        };

        let file = File::open(credential_path).expect("file not found");
        let reader = BufReader::new(file);
        let deserialize: ServiceData = serde_json::from_reader(reader).unwrap();

        let url = deserialize.url;
        let auth_key = deserialize.authentication_key;

        Ok((url, auth_key))
    }
}
