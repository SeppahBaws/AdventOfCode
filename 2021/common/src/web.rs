use anyhow::Result;
use reqwest::{Method, Request, Url};

pub async fn get_day_input(year: u32, day: u32, cookie: String) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut request = Request::new(Method::GET, Url::parse(url.as_str())?);
    request.headers_mut().insert("session", cookie.parse()?);
    let client = reqwest::Client::builder().build()?;
    let res = client.execute(request).await?;
    let body = res.text().await?;

    Ok(body)
}
