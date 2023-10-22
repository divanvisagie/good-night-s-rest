use crate::{requests::Request, method::Method};
use crate::openapi::OpenAPI;

pub struct Collection {
    pub name: String,
    pub requests: Vec<Request>,
    pub servers: Vec<String>,
}

impl Collection {
    pub fn new(name: String, collection: Vec<Request>) -> Collection {
        Collection {
            name,
            requests: collection,
            servers: Vec::new(),
        }
    }

    pub fn from_openapi_format(openapi: OpenAPI) -> Collection {
        let mut collection = Collection::new(openapi.info.title, Vec::new());
        for (path, method) in openapi.paths.iter() {
            for (method_name, _) in method.iter() {
                let mut request = Request::new();
                request.url = path.clone();
                request.method = Method::from_string(method_name.clone());
                collection.requests.push(request);
            }
        }
        for server in openapi.servers.iter() {
            collection.servers.push(server.url.clone());
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
        assert_eq!(collection.requests.len(), 19);
    }
}
