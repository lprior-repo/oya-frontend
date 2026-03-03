pub struct RestateClient {
    base_url: String,
}

impl RestateClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_invocations(&self) -> Result<wasm_bindgen::JsValue, String> {
        use wasm_bindgen::JsCast;

        let window = web_sys::window().ok_or("No window")?;
        let response = window
            .fetch_with_str(&format!("{}/invocations", self.base_url))
            .map_err(|e| format!("Fetch error: {:?}", e))?
            .await
            .map_err(|e| format!("Promise error: {:?}", e))?;

        if response.ok() {
            response.json().map_err(|e| format!("JSON error: {:?}", e))
        } else {
            Err(format!("Request failed: {}", response.status()))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_invocations(&self) -> Result<String, String> {
        Ok("[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restate_client_new() {
        let client = RestateClient::new("http://localhost:8080");
        assert_eq!(client.base_url(), "http://localhost:8080");
    }

    #[test]
    fn test_restate_client_empty_url() {
        let client = RestateClient::new("");
        assert_eq!(client.base_url(), "");
    }

    #[test]
    fn test_restate_client_https_url() {
        let client = RestateClient::new("https://restate.example.com");
        assert_eq!(client.base_url(), "https://restate.example.com");
    }

    #[test]
    fn test_restate_client_base_url_immutability() {
        let client = RestateClient::new("http://localhost:8080");
        let url = client.base_url();
        assert_eq!(url, "http://localhost:8080");
    }
}
