use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::requests::Request;

pub type Collection = Vec<Request>;

pub struct CollectionItem {
    pub name: String,
    pub collection: Collection,
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
    pub fn new(name: String, collection: Collection) -> CollectionItem {
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


#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::method::Method;
    use super::*;

    #[test]
    fn test_to_openapi_format() {
        let mut collection = Collection::new();
        let request = Request {
            url: String::from("https://httpbin.org/get"),
            body: String::from(""),
            method: Method::GET,
            headers: vec![],
            query_params: vec![],
        };
        collection.push(request);
        let collection_item = CollectionItem::new(String::from("Test"), collection);
        let openapi = collection_item.to_openapi_format();
        
        // write to the /out directory
        let mut file = std::fs::File::create("./out/openapi.yaml").unwrap();
        file.write_all(openapi.as_bytes()).unwrap();
    }
}
