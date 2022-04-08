use curl::easy::Easy2;
use serde::{Deserialize, Serialize};
use std::env;

mod handler;
use handler::request_handler::{Collector, Query, TargetLanguage, Url};
use handler::service_handler::{ServiceData, ServiceType};

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

    let sd = ServiceData {
        url: "".to_string(),
        authentication_key: "".to_string(),
    };
    let service_data: ServiceData = ServiceData::new(&sd, _service_type);

    let url: Url = Url {
        url: service_data.url,
        //url: "https://www.rust-lang.org/".to_string(),
    };

    let tmp_params = Query {
        auth_key: service_data.authentication_key,
        text: "Hello".to_string(),
        target_lang: TargetLanguage::JA,
    };

    println!("{:?}", tmp_params);
    tmp_params.run_curl(url);

    // HTTPが動かない
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
        assert_eq!(
            ServiceType::Deepl,
            ServiceType::load_service_type(&args_vec[0])
        );
    }

    #[test]
    fn test_load_service_data() {
        let _service_type = ServiceType::Deepl;
        let sd = ServiceData {
            url: "".to_string(),
            authentication_key: "".to_string(),
        };
        let service_data: ServiceData = ServiceData::new(&sd, _service_type);
        let expected = ServiceData {
            url: "https://api-free.deepl.com/v2/translate".to_string(),
            authentication_key: "819ee1f7-f31b-1bc8-03a6-5e5a33b7bbd5:fx".to_string(),
        };
        assert_eq!(expected, service_data);
    }

    #[test]
    fn test_generate_query() {
        let expected = String::from("url?auth_key=auth_key?text?hello?target_lang=JA");
        //        assert_eq!(expected, http_request.generate_query());
    }
}
