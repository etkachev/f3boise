use crate::shared::common_errors::AppError;
use crate::web_api_routes::auth::internal_auth::BOISE_KEY;
use url::Url;

pub async fn trigger_remind_missing_bb(base_url: &str) -> Result<(), AppError> {
    let url_call = build_url(base_url, "/back_blasts/remind-missing-bb");
    let client = build_client()?;
    let response = client.get(url_call).send().await?;
    if response.status().is_success() {
        println!("Successful trigger for remind missing bb");
    }
    Ok(())
}

pub async fn trigger_leaderboard_stats(base_url: &str) -> Result<(), AppError> {
    let url_call = build_url(base_url, "/pax/post-leaderboard");
    let client = build_client()?;
    let response = client.get(url_call).send().await?;
    if response.status().is_success() {
        println!("Successful leaderboard check");
    } else {
        println!("Unsuccessful leaderboard check");
    }

    Ok(())
}

fn build_client() -> Result<reqwest::Client, AppError> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-F3Boise-Key",
        reqwest::header::HeaderValue::from_static(BOISE_KEY),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

fn build_url(base_url: &str, path: &str) -> Url {
    Url::parse(format!("http://{}{}", base_url, path).as_str())
        .unwrap_or_else(|_| Url::parse(base_url).unwrap())
}
