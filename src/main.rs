use curl::easy::{Easy2, Handler, WriteError};
use std::env;
use std::{fmt::Debug};
use std::fmt;

pub mod service_handler;
use service_handler::{ServiceData, ServiceType};


// TODO:
/// - まずはシンプルに動かすMock
/// - 非同期にリファクタ
/// - サービスごとにtrait, 実装ファイルを分割
/// - オブジェクト指向にリファクタ
/// - セキュアバイ・デザイン: ドメインプリミティブ型に変更
///

#[derive(Debug, Clone, Copy)]
enum TargetLanguage {
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

#[derive(Debug)]
struct HttpRequest{
    pub serviceData: ServiceData,
    pub text: String,
    pub target_lang: TargetLanguage,
}

impl HttpRequest {
    fn generate_request(&self)  -> String{
        let url = &self.serviceData.url;
        let auth_key = &self.serviceData.authentication_key;
        let text = &self.text;
        let target_lang = self.target_lang.to_string();

        let full = format!("{}?{}?{}?{}", url, auth_key, text, target_lang);
        full.to_string()
    }
}

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let _service_type = ServiceType::load_service_type(&args[0]);

    let sd = ServiceData{url: "".to_string(), authentication_key: "".to_string()};
    let service_data:ServiceData = ServiceData::new(&sd , _service_type);



    let tmp_request = HttpRequest{
        serviceData: service_data,
        text: "Hello Wold".to_string(),
        target_lang: TargetLanguage::JP
    };

    println!("{:?}", tmp_request);

    //let mut easy = Easy2::new(Collector(Vec::new()));
    //easy.post(true).unwrap();
    //easy.url().unwrap();

    //assert_eq!(easy.response_code().unwrap(), 200);
    //let contents = easy.get_ref();
    //println!("{}", String::from_utf8_lossy(&contents.0));;
}
// Test 
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_load_service_type() {
        let args_vec = vec!["deepl".to_string()];
        assert_eq!(ServiceType::Deepl, ServiceType::load_service_type(&args_vec[0]));
    }

    #[test]
    fn test_load_service_data() {
        let _service_type = ServiceType::Deepl;
        let sd = ServiceData{url: "".to_string(), authentication_key: "".to_string()};
        let service_data:ServiceData = ServiceData::new(&sd , _service_type);
        let correct_data = ServiceData {
            url:"https://api-free.deepl.com/v2/translate".to_string(),
            authentication_key: "819ee1f7-f31b-1bc8-03a6-5e5a33b7bbd5:fx".to_string(),
        };
        assert_eq!(correct_data, service_data);
    
    }
}
