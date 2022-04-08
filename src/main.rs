use std::env;
use serde::{Serialize, Deserialize};
use curl::easy::{Easy2};

mod handler;
use handler::service_handler::{ServiceData, ServiceType};
use handler::request_handler::{HttpRequest,TargetLanguage, Collector};


// TODO:
/// - まずはシンプルに動かすMock
/// - 非同期にリファクタ
/// - サービスごとにtrait, 実装ファイルを分割
/// - オブジェクト指向にリファクタ
/// - 変数型をドメインプリミティブ型に変更
/// - 依存の方向性を整理する

fn main() {
    let args: Vec<String> = env::args().collect();

    let _service_type = ServiceType::load_service_type(&args[0]);

    let sd = ServiceData{url: "".to_string(), authentication_key: "".to_string()};
    let service_data:ServiceData = ServiceData::new(&sd , _service_type);



    let tmp_request = HttpRequest{
        service_data: service_data,
        text: "Hello Wold".to_string(),
        target_lang: TargetLanguage::JP
    };

    let query: String = HttpRequest::generate_query(&tmp_request);
    println!("{}", &query);


    let encoded: Vec<u8> = Serialize::serialize(&self, serializer);
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.post(true).unwrap();
    //easy.http_version(curl::easy::HttpVersion::V10);
    easy.post_fields_copy()
    easy.url(&query).unwrap();

    println!("{}", &easy.response_code().unwrap());
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
