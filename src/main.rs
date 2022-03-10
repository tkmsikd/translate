use std::env;
use std::fs::File;
use std::{fmt::Debug, io::BufReader};
use serde::de::value::Error;
use serde_json;
use serde::Deserialize;


#[allow(unused)]
#[derive(Debug, PartialEq, Deserialize)]
struct ServiceData {
    url: String,
    authentication_key: String,
}

#[derive(Debug, PartialEq)]
enum ServiceType {
    Deepl,
    GoogleTranslate,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _service_type = load_service_type(&args);

    println!("{:?}", _service_type);
    let _service_data = load_service_data(_service_type).unwrap();

    println!("{:?}",_service_data.url);


}

fn load_service_type(args: &Vec<String>) -> ServiceType {
    let service = args[0].as_str();
    match service {
        "google" => ServiceType::GoogleTranslate,
        "deepl" | _ => ServiceType::Deepl,
    }
}

fn load_service_data(service_type: ServiceType) -> Result<ServiceData, Box<Error>> {
    let credential_path = match service_type {
        ServiceType::Deepl => "./src/deepl_credential.json",
        ServiceType::GoogleTranslate => "./src/google_translate_credential.json",
    };

    let file = File::open(credential_path).expect("file not found");
    let reader = BufReader::new(file);
    let deserialize = serde_json::from_reader(reader).unwrap();

    Ok(deserialize)
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_load_service_type() {
        let service_type: ServiceType = ServiceType::Deepl;
        let args_vec = vec!["deepl".to_string()];
        assert_eq!(service_type, load_service_type(&args_vec));
    }

    #[test]
    fn test_load_service_data() {
        let placeholder_data: ServiceData = ServiceData {
            url: "placeholder".to_string(),
            authentication_key: "placeholder".to_string(),
        };

        let service_type = ServiceType::Deepl;

        assert_eq!(placeholder_data, load_service_data(service_type).unwrap());
    }
}
