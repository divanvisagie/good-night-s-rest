use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName};

use crate::method::Method;

#[derive(Clone, PartialEq)]
pub struct Request {
    pub url: String,
    pub body: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub query_params: Vec<(String, String)>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            url: "https://httpbin.org/get".to_string(),
            body: "".to_string(),
            method: Method::GET,
            headers: vec![],
            query_params: vec![],
        }
    }
}

pub async fn perform_request(
    req: Request
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let headers = req.headers
        .iter()
        .fold(HeaderMap::new(), |mut acc, (k, v)| {
            let header_name = HeaderName::from_str(k).unwrap();
            acc.insert(header_name, v.parse().unwrap());
            acc
        });

    let response = match req.method {
        Method::GET => {
            client
                .get(req.url)
                .headers(headers)
                .query(&req.query_params)
                .send()
                .await?
        }
        Method::POST => client.post(req.url).body(req.body.to_string()).send().await?,
        Method::PUT => client.put(req.url).body(req.body.to_string()).send().await?,
        Method::DELETE => client.delete(req.url).body(req.body.to_string()).send().await?,
    };
    Ok(response.text().await?)
}
