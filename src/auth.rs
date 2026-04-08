use crate::config::DEFAULT_BASE_URL;
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const CALLBACK_PORT: u16 = 8697;
const OAUTH_TIMEOUT_SECS: u64 = 300; // 5 minutes
const DEFAULT_COGNITO_DOMAIN: &str = "auth.jpx-jquants.com";
const DEFAULT_COGNITO_CLIENT_ID: &str = "3p2n2njg72hq4emn9lr1hksva2";

struct CognitoConfig {
    domain: String,
    client_id: String,
    scopes: String,
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    api_key: String,
    saved_at: u64,
}

#[derive(Deserialize)]
struct ApiKeyResponse {
    #[serde(rename = "apiKey")]
    api_key: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    id_token: String,
    #[allow(dead_code)]
    access_token: String,
    #[allow(dead_code)]
    refresh_token: Option<String>,
}

fn credentials_file_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".config/jquants/credentials.json"))
}

fn cognito_config_from_env() -> CognitoConfig {
    let domain =
        std::env::var("COGNITO_DOMAIN").unwrap_or_else(|_| DEFAULT_COGNITO_DOMAIN.to_string());
    let client_id = std::env::var("COGNITO_CLIENT_ID")
        .unwrap_or_else(|_| DEFAULT_COGNITO_CLIENT_ID.to_string());
    let scopes = std::env::var("COGNITO_SCOPES").unwrap_or_else(|_| "openid".to_string());
    CognitoConfig {
        domain,
        client_id,
        scopes,
    }
}

fn generate_code_verifier() -> String {
    let mut bytes = [0u8; 96];
    rand::rng().fill(&mut bytes[..]);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn compute_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

fn build_authorize_url(config: &CognitoConfig, challenge: &str) -> String {
    let redirect_uri = format!("http://localhost:{}/callback", CALLBACK_PORT);
    let mut url = reqwest::Url::parse(&format!("https://{}/oauth2/authorize", config.domain))
        .expect("invalid domain");
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", &config.client_id)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("scope", &config.scopes)
        .append_pair("code_challenge", challenge)
        .append_pair("code_challenge_method", "S256");
    url.to_string()
}

fn build_logout_url(config: &CognitoConfig) -> String {
    let logout_uri = format!("http://localhost:{}/logout", CALLBACK_PORT);
    let mut url =
        reqwest::Url::parse(&format!("https://{}/logout", config.domain)).expect("invalid domain");
    url.query_pairs_mut()
        .append_pair("client_id", &config.client_id)
        .append_pair("logout_uri", &logout_uri);
    url.to_string()
}

fn extract_code_from_request(request: &str) -> Result<String, AppError> {
    let first_line = request.lines().next().unwrap_or("");
    let query_start = first_line
        .find('?')
        .ok_or_else(|| AppError::Auth("No query string in callback request".into()))?;
    let query_end = first_line[query_start..]
        .find(' ')
        .map(|i| query_start + i)
        .unwrap_or(first_line.len());
    let query_string = &first_line[query_start + 1..query_end];

    for param in query_string.split('&') {
        if let Some(value) = param.strip_prefix("code=") {
            return Ok(value.to_string());
        }
        if param.starts_with("error=") {
            let error_val = param.get(6..).unwrap_or("unknown");
            return Err(AppError::Auth(format!(
                "Authorization denied: {}",
                error_val
            )));
        }
    }
    Err(AppError::Auth("No authorization code in callback".into()))
}

async fn bind_callback_listener() -> Result<tokio::net::TcpListener, AppError> {
    tokio::net::TcpListener::bind(format!("127.0.0.1:{}", CALLBACK_PORT))
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                AppError::Io(std::io::Error::new(
                    e.kind(),
                    format!(
                        "Port {} is already in use. Please close the application using that port.",
                        CALLBACK_PORT
                    ),
                ))
            } else {
                AppError::Io(e)
            }
        })
}

async fn send_html_response(stream: &mut tokio::net::TcpStream, body: &str) {
    use tokio::io::AsyncWriteExt;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\
        Content-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes()).await;
    let _ = stream.flush().await;
}

async fn wait_for_callback(listener: tokio::net::TcpListener) -> Result<String, AppError> {
    use tokio::io::AsyncReadExt;
    use tokio::time::{timeout, Duration};

    timeout(Duration::from_secs(OAUTH_TIMEOUT_SECS), async {
        let (mut stream, _) = listener.accept().await?;

        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).await?;
        let request = String::from_utf8_lossy(&buf[..n]);

        let code = extract_code_from_request(&request)?;

        send_html_response(
            &mut stream,
            "<html><body><h1>Login successful!</h1>\
            <p>You can now close this window and return to the terminal.</p>\
            </body></html>",
        )
        .await;

        Ok::<String, AppError>(code)
    })
    .await
    .map_err(|_| AppError::Auth("Login timed out after 5 minutes. Please try again.".into()))?
}

async fn wait_for_logout_callback(listener: tokio::net::TcpListener) -> Result<(), AppError> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::time::{timeout, Duration};

    timeout(Duration::from_secs(OAUTH_TIMEOUT_SECS), async {
        loop {
            let (mut stream, _) = listener.accept().await?;

            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).await?;
            let request = String::from_utf8_lossy(&buf[..n]);
            let first_line = request.lines().next().unwrap_or("");

            // Ignore connections that are not the Cognito logout redirect
            if !first_line.starts_with("GET /logout") {
                let _ = stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                    .await;
                continue;
            }

            send_html_response(
                &mut stream,
                "<html><body><h1>Logout successful!</h1>\
                <p>You can now close this window and return to the terminal.</p>\
                </body></html>",
            )
            .await;

            return Ok::<(), AppError>(());
        }
    })
    .await
    .map_err(|_| AppError::Auth("Logout timed out after 5 minutes. Please try again.".into()))?
}

async fn post_token_endpoint(
    client: &reqwest::Client,
    domain: &str,
    params: &[(&str, &str)],
    error_prefix: &str,
) -> Result<TokenResponse, AppError> {
    let response = client
        .post(format!("https://{}/oauth2/token", domain))
        .form(params)
        .send()
        .await?;

    let status = response.status().as_u16();
    if status != 200 {
        let text = response.text().await.unwrap_or_default();
        return Err(AppError::Auth(format!(
            "{} ({}): {}",
            error_prefix, status, text
        )));
    }

    Ok(response.json().await?)
}

async fn exchange_code_for_id_token(
    client: &reqwest::Client,
    config: &CognitoConfig,
    code: &str,
    verifier: &str,
) -> Result<String, AppError> {
    let redirect_uri = format!("http://localhost:{}/callback", CALLBACK_PORT);
    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", config.client_id.as_str()),
        ("code", code),
        ("redirect_uri", redirect_uri.as_str()),
        ("code_verifier", verifier),
    ];
    let token_response =
        post_token_endpoint(client, &config.domain, &params, "Token exchange failed").await?;
    Ok(token_response.id_token)
}

async fn post_api_key(
    client: &reqwest::Client,
    base_url: &str,
    id_token: &str,
) -> Result<String, AppError> {
    let url = format!("{}/cli/api-key", base_url);
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await?;

    let status = response.status().as_u16();
    if status != 200 {
        let text = response.text().await.unwrap_or_default();
        if status == 403 && text.to_ascii_lowercase().contains("no active subscription") {
            return Err(AppError::AuthSubscription(format!(
                "Failed to obtain API key ({}): {}",
                status, text
            )));
        }
        return Err(AppError::Auth(format!(
            "Failed to obtain API key ({}): {}",
            status, text
        )));
    }

    let body: ApiKeyResponse = response.json().await?;
    Ok(body.api_key)
}

fn current_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn save_credentials(credentials: &Credentials) -> Result<(), AppError> {
    let creds_path = credentials_file_path()
        .ok_or_else(|| AppError::Config("Cannot determine home directory".into()))?;

    if let Some(dir) = creds_path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let json = serde_json::to_string_pretty(credentials)?;
    std::fs::write(&creds_path, &json)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&creds_path)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(&creds_path, perms)?;
    }

    Ok(())
}

fn load_credentials() -> Option<Credentials> {
    let path = credentials_file_path()?;
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn remove_credentials() -> Result<Option<std::path::PathBuf>, AppError> {
    if let Some(path) = credentials_file_path().filter(|p| p.exists()) {
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(AppError::Io(e)),
        }
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

pub async fn logout() -> Result<(), AppError> {
    // Bind listener before opening browser to avoid race condition
    let listener = bind_callback_listener().await?;

    // Clear Cognito browser session (always, regardless of local credentials)
    let config = cognito_config_from_env();
    let logout_url = build_logout_url(&config);
    eprintln!("Opening browser to clear Cognito session...");
    if let Err(e) = open::that(&logout_url) {
        eprintln!("Warning: Failed to open browser ({})", e);
        eprintln!("Please open the following URL in your browser to clear the session:");
        eprintln!("{}", logout_url);
    }

    eprintln!(
        "Waiting for logout callback on port {} ({} minute timeout)...",
        CALLBACK_PORT,
        OAUTH_TIMEOUT_SECS / 60
    );
    let cognito_result = wait_for_logout_callback(listener).await;
    if let Err(ref e) = cognito_result {
        eprintln!("Warning: Cognito session may not be fully cleared: {}", e);
    }

    // Remove local credentials if they exist
    match remove_credentials()? {
        Some(path) => {
            println!("Logged out. Credentials removed from {}", path.display());
        }
        None => {
            println!("Logged out. (No local credentials to remove.)");
        }
    }
    Ok(())
}

pub async fn login() -> Result<(), AppError> {
    let config = cognito_config_from_env();
    let base_url =
        std::env::var("JQUANTS_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    let verifier = generate_code_verifier();
    let challenge = compute_code_challenge(&verifier);
    let authorize_url = build_authorize_url(&config, &challenge);

    // Bind listener before opening browser to avoid race condition
    let listener = bind_callback_listener().await?;

    eprintln!("Opening browser for login...");
    if let Err(e) = open::that(&authorize_url) {
        eprintln!("Warning: Failed to open browser ({})", e);
        eprintln!("Please open the following URL in your browser:");
        eprintln!("{}", authorize_url);
    }

    eprintln!(
        "Waiting for callback on port {} ({} minute timeout)...",
        CALLBACK_PORT,
        OAUTH_TIMEOUT_SECS / 60
    );
    let code = wait_for_callback(listener).await?;

    eprintln!("Exchanging authorization code for tokens...");
    let http_client = reqwest::Client::new();
    let id_token = exchange_code_for_id_token(&http_client, &config, &code, &verifier).await?;

    eprintln!("Obtaining API key...");
    let api_key = post_api_key(&http_client, &base_url, &id_token).await?;

    let credentials = Credentials {
        api_key,
        saved_at: current_unix_secs(),
    };
    save_credentials(&credentials)?;

    println!("Login successful! Credentials saved to ~/.config/jquants/credentials.json");
    Ok(())
}

#[allow(clippy::unused_async)]
pub async fn resolve_auth() -> Result<String, AppError> {
    // 1. Try saved credentials
    if let Some(creds) = load_credentials() {
        return Ok(creds.api_key);
    }

    // 2. Fall back to API key env var
    if let Ok(api_key) = std::env::var("JQUANTS_API_KEY") {
        return Ok(api_key);
    }

    // 3. Neither available
    Err(AppError::Auth("No valid authentication found".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_cognito_config_uses_defaults() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("COGNITO_DOMAIN");
        std::env::remove_var("COGNITO_CLIENT_ID");
        std::env::remove_var("COGNITO_SCOPES");

        let config = cognito_config_from_env();
        assert_eq!(config.domain, DEFAULT_COGNITO_DOMAIN);
        assert_eq!(config.client_id, DEFAULT_COGNITO_CLIENT_ID);
        assert_eq!(config.scopes, "openid");
    }

    #[test]
    fn test_cognito_config_env_override() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::set_var("COGNITO_DOMAIN", "custom.auth.example.com");
        std::env::set_var("COGNITO_CLIENT_ID", "custom_client_id");
        std::env::set_var("COGNITO_SCOPES", "openid profile");

        let config = cognito_config_from_env();
        assert_eq!(config.domain, "custom.auth.example.com");
        assert_eq!(config.client_id, "custom_client_id");
        assert_eq!(config.scopes, "openid profile");

        std::env::remove_var("COGNITO_DOMAIN");
        std::env::remove_var("COGNITO_CLIENT_ID");
        std::env::remove_var("COGNITO_SCOPES");
    }

    #[test]
    fn test_generate_code_verifier_length() {
        let verifier = generate_code_verifier();
        // 96 bytes → base64url (no padding) = 128 chars
        assert_eq!(verifier.len(), 128);
    }

    #[test]
    fn test_generate_code_verifier_unique() {
        let v1 = generate_code_verifier();
        let v2 = generate_code_verifier();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_generate_code_verifier_url_safe() {
        let verifier = generate_code_verifier();
        assert!(verifier
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn test_compute_code_challenge_no_padding() {
        let challenge = compute_code_challenge("test_verifier");
        assert!(!challenge.contains('+'));
        assert!(!challenge.contains('/'));
        assert!(!challenge.contains('='));
    }

    #[test]
    fn test_compute_code_challenge_deterministic() {
        let verifier = "fixed_verifier_for_test";
        assert_eq!(
            compute_code_challenge(verifier),
            compute_code_challenge(verifier)
        );
    }

    #[test]
    fn test_build_logout_url_format() {
        let config = CognitoConfig {
            domain: "auth.example.com".into(),
            client_id: "test_client_id".into(),
            scopes: "openid".into(),
        };
        let url = build_logout_url(&config);
        assert!(url.starts_with("https://auth.example.com/logout?"));
        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("logout_uri="));
        assert!(url.contains("8697"));
        assert!(url.contains("%2Flogout"));
    }

    #[test]
    fn test_build_logout_url_uses_config_domain() {
        let config = CognitoConfig {
            domain: "custom.auth.region.amazoncognito.com".into(),
            client_id: "my_client".into(),
            scopes: "openid".into(),
        };
        let url = build_logout_url(&config);
        assert!(url.starts_with("https://custom.auth.region.amazoncognito.com/logout?"));
        assert!(url.contains("client_id=my_client"));
        assert!(url.contains("logout_uri="));
    }

    #[test]
    fn test_build_authorize_url_contains_required_params() {
        let config = CognitoConfig {
            domain: "auth.example.com".into(),
            client_id: "test_client_id".into(),
            scopes: "openid".into(),
        };
        let url = build_authorize_url(&config, "test_challenge");
        assert!(url.starts_with("https://auth.example.com/oauth2/authorize?"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("code_challenge=test_challenge"));
        assert!(url.contains("code_challenge_method=S256"));
        assert!(url.contains("8697"));
    }

    #[test]
    fn test_extract_code_from_request_success() {
        let req = "GET /callback?code=authcode123&state=xyz HTTP/1.1\r\nHost: localhost\r\n\r\n";
        assert_eq!(extract_code_from_request(req).unwrap(), "authcode123");
    }

    #[test]
    fn test_extract_code_from_request_error_param() {
        let req = "GET /callback?error=access_denied HTTP/1.1\r\n";
        assert!(matches!(
            extract_code_from_request(req).unwrap_err(),
            AppError::Auth(_)
        ));
    }

    #[test]
    fn test_extract_code_from_request_no_query() {
        let req = "GET /callback HTTP/1.1\r\n";
        assert!(matches!(
            extract_code_from_request(req).unwrap_err(),
            AppError::Auth(_)
        ));
    }

    #[test]
    fn test_credentials_round_trip() {
        let creds = Credentials {
            api_key: "test-api-key-12345".into(),
            saved_at: 1711234567,
        };
        let json = serde_json::to_string(&creds).unwrap();
        let deserialized: Credentials = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.api_key, "test-api-key-12345");
        assert_eq!(deserialized.saved_at, 1711234567);
    }

    #[test]
    fn test_api_key_response_deserialization() {
        let json = r#"{"apiKey":"abc123","created":true}"#;
        let response: ApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.api_key, "abc123");
    }

    #[test]
    fn test_credentials_file_path() {
        let path = credentials_file_path().unwrap();
        let path_str = path.to_string_lossy();
        assert!(path_str.contains(".config/jquants/credentials.json"));
    }

    #[test]
    fn test_logout_removes_file() {
        let _guard = ENV_MUTEX.lock().unwrap();

        let home = std::env::temp_dir().join("jquants_test_logout_home");
        let creds_dir = home.join(".config/jquants");
        std::fs::create_dir_all(&creds_dir).unwrap();
        let creds_path = creds_dir.join("credentials.json");
        let creds = Credentials {
            api_key: "test-key".into(),
            saved_at: 0,
        };
        std::fs::write(&creds_path, serde_json::to_string(&creds).unwrap()).unwrap();
        assert!(creds_path.exists());

        std::env::set_var("HOME", &home);
        let result = remove_credentials().unwrap();
        std::env::remove_var("HOME");

        assert!(result.is_some());
        assert!(!creds_path.exists());
    }

    #[test]
    fn test_logout_nonexistent_is_noop() {
        let _guard = ENV_MUTEX.lock().unwrap();

        let home = std::env::temp_dir().join("jquants_test_logout_noop_home");
        std::fs::create_dir_all(&home).unwrap();
        let creds_path = home.join(".config/jquants/credentials.json");
        assert!(!creds_path.exists());

        std::env::set_var("HOME", &home);
        let result = remove_credentials().unwrap(); // ファイルなしでも Ok(None) を返すこと
        std::env::remove_var("HOME");

        assert!(result.is_none());
    }
}
