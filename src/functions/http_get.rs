use reqwest::{Error, Response};

pub async fn http_get(url: &str) -> Result<(), Error> {
    let res: Response = reqwest::get(url).await?;
    println!("Status: {}", res.status());
    println!("Remote address: {:?}", res. remote_addr());
    println!("Body:\n\n{:?}", res.text().await?);
    Ok(())
}
