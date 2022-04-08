use crate::handler::service_handler::ServiceData;
use bincode;
use curl::easy::{Easy2, Handler, WriteError};
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::process::Command;
#[allow(unused)]

pub struct Collector(pub Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TargetLanguage {
    JA,
    EN,
    Undefined,
}

impl fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetLanguage::JA => write!(f, "JA"),
            TargetLanguage::EN => write!(f, "EN"),
            TargetLanguage::Undefined => write!(f, "undefined"),
        }
    }
}

pub struct Url {
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    //pub service_data: ServiceData,
    pub auth_key: String,
    pub text: String,
    pub target_lang: TargetLanguage,
}

impl Query {
    pub fn encode_query<T: Serialize>(&self, target: &T) -> Vec<u8> {
        bincode::serialize(&target).unwrap()
    }

    pub fn run_curl(&self, url: Url) {
        let url = url.url;
        let target_lang = "JP"; //FIXME: TargetLanguageの型から変換して使えるように修正する
        let output = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            Command::new("curl")
                .args([
                    &url,
                    "--verbose",
                    "-d",
                    &self.auth_key,
                    "-d",
                    &self.text,
                    "-d",
                    target_lang,
                ])
                .output()
                .expect("Error: Something went wrong while executing curl")
        } else {
            panic!("Not available on Windows OS")
        };

        let translate = output.stdout;
        println!("{:?}", translate);
    }

    pub fn post_request(&self) {
        //TODO: implement http post not using curl command
    }

}