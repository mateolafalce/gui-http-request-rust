use reqwest::{
    Error,
    Response,
    Client,
    Body
};

pub async fn http_request(
    method: i32,
    url: String,
    body: String,
    admin: String,
    password: String
) -> Result<String, Error> {
    let client: Client = Client::new();
    match method {
        0 => {
            let res: Response = client.get(&url).send().await?;
            return Ok(res.text().await?)
        },
        1 => {
            let res: Response = client.post(&url)
            .body(Body::from(body))
            .send()
            .await?;
            return Ok(res.text().await?)
        },
        2 => {
            let res = client
            .put(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
            return Ok(res.text().await?)
        },
        3 => {
            let res = client.delete(&url)
            .basic_auth(&admin, Some(&password))
            .send()
            .await?;
            return Ok(res.text().await?)
        },
        _ => unreachable!(),
    }
}
