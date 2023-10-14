use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Info {
    title: String,
}
impl Info {
    pub fn new(title: String) -> Info {
        Info { title }
    }
}

#[derive(Serialize, Deserialize)]
struct CodeResponseObject {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct RequestBodyObject { 
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct MethodType {
   requestBody: Option<RequestBodyObject>,
   responses: HashMap<String, CodeResponseObject>,
}

#[derive(Serialize, Deserialize)]
struct OpenAPI {
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

    fn load_from_yaml_file(path: String) -> OpenAPI {
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
    use super::*;
    use crate::method::Method;
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
            }
            None => {
                panic!("Key not found");
            }
        }

    }
}
