use crate::errors::ApiError;

pub async fn do_fetch(url: &str) -> Result<String, ApiError> {
  // let proxy = reqwest::Proxy::http("https://my.prox").unwrap();
  let client = reqwest::Client::builder()
    // .proxy(proxy)
    .build()
    .unwrap();

  let resp = client.get(url).send().await;
  if resp.is_err() {
    let error_msg = format!(
      "received error while fetching news (code='{}')...",
      resp.as_ref().unwrap().status()
    );
    Err(ApiError::new(100, error_msg))
  } else {
    Ok(resp.unwrap().text().await.unwrap())
  }
}

pub mod fixtures;
pub mod news;
