use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::requests::Request;

// pub type Collection = Vec<Request>;

pub struct CollectionItem {
    pub name: String,
    pub collection: Vec<Request>,
}

#[derive(Serialize, Deserialize)]
struct Info { 
    title: String,
}
impl Info {
    pub fn new(title: String) -> Info {
        Info {
            title,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MethodType {
    response: String
}

#[derive(Serialize, Deserialize)]
struct OpenAPI {
    pub openapi: String,
    pub info: Info,
    pub paths: HashMap<String, HashMap<String, MethodType>>
}

impl OpenAPI {
    pub fn new(title: String) -> OpenAPI {
        OpenAPI {
            openapi: String::from("3.0.1"),
            info: Info::new(title),
            paths: HashMap::new(),
        }
    }
}

impl CollectionItem {
    pub fn new(name: String, collection: Vec<Request>) -> CollectionItem {
        CollectionItem {
            name,
            collection,
        }
    }

    pub fn to_openapi_format(&self) -> String {
        let mut openapi = OpenAPI::new(self.name.clone());
        let mut path_map: HashMap<String, HashMap<String, MethodType>> = HashMap::new();
        for request in self.collection.iter() {
            let mut p: HashMap<String, MethodType> = HashMap::new();
            p.insert(request.method.to_string(), MethodType { response: "200".to_string() });
            path_map.insert(request.url.clone(), p);
        }
        openapi.paths = path_map;
        serde_yaml::to_string(&openapi).unwrap()
    }
}

