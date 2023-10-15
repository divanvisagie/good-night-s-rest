use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub title: String,
}
impl Info {
    pub fn new(title: String) -> Info {
        Info { title }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CodeResponseObject {
   pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBodyObject { 
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MethodType {
   pub requestBody: Option<RequestBodyObject>,
   pub responses: HashMap<String, CodeResponseObject>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAPI {
    pub openapi: String,
    pub info: Info,
    pub paths: HashMap<String, HashMap<String, MethodType>>,
}

impl OpenAPI {
    pub fn new(title: String) -> OpenAPI {
        OpenAPI {
            openapi: String::from("3.0.1"),
            info: Info::new(title),
            paths: HashMap::new(),
        }
    }

    pub fn load_from_yaml_file(path: String) -> OpenAPI {
        // load the file first
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        match serde_yaml::from_reader(reader) {
            Ok(openapi) => {
                return openapi;
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{method::Method, openapi::OpenAPI};
    use std::io::Write;

    #[test]
    fn test_to_openapi_format() {
        let path = String::from("./test_data/test.yaml");
        let collection = OpenAPI::load_from_yaml_file(path);
        assert_eq!(collection.openapi, String::from("3.1.0"));
        assert_eq!(
            collection.info.title,
            String::from("Swagger Petstore - OpenAPI 3.1")
        );

        match collection.paths.get_key_value("/pet") {
            Some((key, value)) => {
                assert_eq!(key, "/pet");
                assert_eq!(value.len(), 2);
                assert_eq!(value.get("put").unwrap().responses.len(), 4);
                assert_eq!(value.get("put").unwrap().requestBody.is_some(), true);
            }
            None => {
                panic!("Key not found");
            }
        }

    }
}
