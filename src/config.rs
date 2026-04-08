use crate::auth;
use crate::error::AppError;

pub const DEFAULT_BASE_URL: &str = "https://api.jquants.com/v2";

pub struct Config {
    pub api_key: String,
    pub base_url: String,
}

impl Config {
    pub async fn from_env() -> Result<Self, AppError> {
        let api_key = auth::resolve_auth().await?;

        let base_url =
            std::env::var("JQUANTS_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Ok(Config { api_key, base_url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Serialize env-var tests to avoid race conditions between threads
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[tokio::test]
    async fn test_config_with_api_key() {
        let _guard = ENV_MUTEX.lock().unwrap();
        // credentials.json が存在する環境でも env var が使われるよう HOME を隔離
        let tmp_home = std::env::temp_dir().join("jquants_test_config_api_key");
        std::fs::create_dir_all(&tmp_home).unwrap();
        std::env::set_var("HOME", &tmp_home);
        std::env::set_var("JQUANTS_API_KEY", "test_key");
        std::env::remove_var("JQUANTS_BASE_URL");
        let config = Config::from_env().await.unwrap();
        assert_eq!(config.base_url, "https://api.jquants.com/v2");
        assert_eq!(config.api_key, "test_key");
        std::env::remove_var("JQUANTS_API_KEY");
        std::env::remove_var("HOME");
    }

    #[tokio::test]
    async fn test_config_custom_base_url() {
        let _guard = ENV_MUTEX.lock().unwrap();
        // credentials.json が存在する環境でも env var が使われるよう HOME を隔離
        let tmp_home = std::env::temp_dir().join("jquants_test_config_base_url");
        std::fs::create_dir_all(&tmp_home).unwrap();
        std::env::set_var("HOME", &tmp_home);
        std::env::set_var("JQUANTS_API_KEY", "test_key");
        std::env::set_var("JQUANTS_BASE_URL", "https://custom.example.com/v2");
        let config = Config::from_env().await.unwrap();
        assert_eq!(config.base_url, "https://custom.example.com/v2");
        std::env::remove_var("JQUANTS_API_KEY");
        std::env::remove_var("JQUANTS_BASE_URL");
        std::env::remove_var("HOME");
    }
}
