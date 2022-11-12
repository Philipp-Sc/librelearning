#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RequestConfig {
    pub endpoint: String,
    pub user_name: String,
    pub password: String,
}
impl RequestConfig {
    pub fn is_initialized(&self) -> bool {
        // starts with http(s)://  and ends with no /
        if self.endpoint.len() == 0 {
            return false;
        }
        true
    }
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            endpoint: "".to_string(),
            user_name: "".to_string(),
            password: "".to_string(),
        }
    }
}
