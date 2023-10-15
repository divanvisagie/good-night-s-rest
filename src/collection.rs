use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{requests::Request, method::Method};
use crate::openapi::OpenAPI;

pub struct Collection {
    pub name: String,
    pub collection: Vec<Request>,
}


impl Collection {
    pub fn new(name: String, collection: Vec<Request>) -> Collection {
        Collection {
            name,
            collection,
        }
    }

    pub fn from_openapi_format(openapi: OpenAPI) -> Collection {
        let mut collection = Collection::new(openapi.info.title, Vec::new());
        for (path, method) in openapi.paths.iter() {
            for (method_name, _) in method.iter() {
                let mut request = Request::new();
                request.url = path.clone();
                request.method = Method::from_string(method_name.clone());
                collection.collection.push(request);
            }
        }
        collection
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_openapi_format() {
        let path = String::from("./test_data/test.yaml");
        let openapi = OpenAPI::load_from_yaml_file(path);
        let collection = Collection::from_openapi_format(openapi);
        assert_eq!(collection.collection.len(), 19);
        // let first_collection_item = &collection.collection[0];
        // assert_eq!(first_collection_item.url, "/api/v1/cluster");
    }
}
