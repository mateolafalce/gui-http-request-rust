use reqwest::{Body, Client, Error};

pub async fn http_request(
    method: i32,
    url: String,
    body: String,
    admin: String,
    password: String,
) -> Result<String, Error> {
    match method {
        0 => Client::new().get(&url),
        1 => Client::new().post(&url).body(Body::from(body)),
        2 => Client::new()
            .put(url)
            .header("Content-Type", "application/json")
            .body(body),
        _ => Client::new()
            .delete(&url)
            .basic_auth(&admin, Some(&password)),
    }
    .send()
    .await?
    .text()
    .await
}
