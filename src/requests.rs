use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName};

use crate::method::Method;

pub async fn perform_request(
    url: &str,
    method: Method,
    body: &str,
    headers: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let headers = headers
        .iter()
        .fold(HeaderMap::new(), |mut acc, (k, v)| {
            let header_name = HeaderName::from_str(k).unwrap();
            acc.insert(header_name, v.parse().unwrap());
            acc
        });

    let response = match method {
        Method::GET => {
            client
                .get(url)
                .headers(headers)
                .query(&query_params)
                .send()
                .await?
        }
        Method::POST => client.post(url).body(body.to_string()).send().await?,
        Method::PUT => client.put(url).body(body.to_string()).send().await?,
        Method::DELETE => client.delete(url).body(body.to_string()).send().await?,
    };
    Ok(response.text().await?)
}
