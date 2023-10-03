use crate::method::Method;

pub async fn perform_request(url: &str, method: Method, body: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = match method {
        Method::GET => client.get(url).send().await?,
        Method::POST => client.post(url).body(body.to_string()).send().await?,
        Method::PUT => client.put(url).body(body.to_string()).send().await?,
        Method::DELETE => client.delete(url).body(body.to_string()).send().await?,
    };
    Ok(response.text().await?)
}
