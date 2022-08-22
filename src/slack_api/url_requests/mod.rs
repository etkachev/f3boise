use url::Url;

pub trait SlackUrlRequest {
    fn get_url_request(&self, base_api: &str) -> Url;
}
