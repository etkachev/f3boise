use url::Url;

pub trait SlackUrlRequest: serde::Serialize {
    fn get_api_url(&self) -> &str;

    /// get url request along with query params
    fn get_url_request(&self, base_api: &str) -> Url
    where
        Self: Sized,
    {
        let params = serde_qs::to_string(self).unwrap_or_else(|_| "".to_string());
        Url::parse(format!("{}{}?{}", base_api, self.get_api_url(), params).as_str())
            .unwrap_or_else(|_| Url::parse(base_api).unwrap())
    }

    /// get url request without query params
    fn get_plain_url_request(&self, base_api: &str) -> Url
    where
        Self: Sized,
    {
        Url::parse(format!("{}{}", base_api, self.get_api_url()).as_str())
            .unwrap_or_else(|_| Url::parse(base_api).unwrap())
    }
}
