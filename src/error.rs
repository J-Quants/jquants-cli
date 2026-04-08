use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error (status {status}): {message}")]
    Api { status: u16, message: String },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authentication error: {0}")]
    AuthSubscription(String),

    #[error("Usage error: {0}")]
    Usage(String),

    #[error("Parquet error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),

    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow_schema::ArrowError),

    #[error("Response decode error: {source}")]
    Decode {
        source: serde_json::Error,
        body: String,
    },
}

impl AppError {
    pub fn why(&self) -> Option<String> {
        match self {
            AppError::Http(e) => {
                if e.is_connect() {
                    Some("接続失敗".into())
                } else if e.is_timeout() {
                    Some("タイムアウト".into())
                } else if e.is_request() {
                    Some("リクエスト構築エラー".into())
                } else {
                    None
                }
            }
            AppError::Api { status, .. } => Some(format!("HTTP {}", status)),
            AppError::Config(_) => Some("環境変数未設定".into()),
            AppError::Json(_) => Some("レスポンスのJSONパースに失敗".into()),
            AppError::Decode { .. } => {
                Some("レスポンスのデコードに失敗（フィールド型/構造の不一致）".into())
            }
            AppError::Auth(_) => Some("認証情報が見つかりません".into()),
            AppError::AuthSubscription(_) => {
                Some("サブスクリプション（プラン選択）が有効ではありません".into())
            }
            _ => None,
        }
    }

    pub fn hint(&self) -> Option<String> {
        match self {
            AppError::Api { status, .. } => match status {
                401 => Some(
                    "`jquants login` でログインするか、JQUANTS_API_KEY を設定してください。"
                        .into(),
                ),
                403 => Some(
                    "APIプランがこのエンドポイントを含まない可能性があります。J-Quantsのサブスクリプションを確認。"
                        .into(),
                ),
                429 => Some("レート制限超過。少し待ってからリトライしてください。".into()),
                _ => None,
            },
            AppError::Config(_) => Some(
                "JQUANTS_API_KEY を環境変数または .env ファイルで設定してください。詳細: https://jpx-jquants.com/"
                    .into(),
            ),
            AppError::Decode { body, .. } => {
                Some(format!("APIレスポンス（先頭500文字）: {}", body))
            }
            AppError::Auth(_) => Some(
                "`jquants login` でログインするか、JQUANTS_API_KEY を設定してください。".into(),
            ),
            AppError::AuthSubscription(_) => Some(
                "J-Quantsサイト (https://jpx-jquants.com/) でプランを選択・契約してください。"
                    .into(),
            ),
            AppError::Http(e) => {
                if e.is_connect() {
                    Some("ネットワーク接続を確認してください。".into())
                } else if e.is_timeout() {
                    Some("リクエストがタイムアウトしました。ネットワーク状態を確認してください。".into())
                } else {
                    Some("ネットワーク接続と JQUANTS_BASE_URL の設定を確認してください。".into())
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_why_api_401() {
        let e = AppError::Api {
            status: 401,
            message: "Unauthorized".into(),
        };
        assert_eq!(e.why(), Some("HTTP 401".into()));
    }

    #[test]
    fn test_why_config() {
        let e = AppError::Config("missing key".into());
        assert_eq!(e.why(), Some("環境変数未設定".into()));
    }

    #[test]
    fn test_why_json() {
        let e = AppError::Json(serde_json::from_str::<i32>("not json").unwrap_err());
        assert_eq!(e.why(), Some("レスポンスのJSONパースに失敗".into()));
    }

    #[test]
    fn test_hint_api_401() {
        let e = AppError::Api {
            status: 401,
            message: "Unauthorized".into(),
        };
        assert!(e.hint().unwrap().contains("JQUANTS_API_KEY"));
    }

    #[test]
    fn test_hint_api_403() {
        let e = AppError::Api {
            status: 403,
            message: "Forbidden".into(),
        };
        assert!(e.hint().unwrap().contains("サブスクリプション"));
    }

    #[test]
    fn test_hint_api_429() {
        let e = AppError::Api {
            status: 429,
            message: "Too Many Requests".into(),
        };
        assert!(e.hint().unwrap().contains("レート制限"));
    }

    #[test]
    fn test_hint_config() {
        let e = AppError::Config("missing key".into());
        assert!(e.hint().unwrap().contains("JQUANTS_API_KEY"));
    }

    #[test]
    fn test_why_auth_subscription() {
        let e = AppError::AuthSubscription("Failed to obtain API key (403): ...".into());
        assert!(e.why().unwrap().contains("サブスクリプション"));
    }

    #[test]
    fn test_hint_auth_subscription() {
        let e = AppError::AuthSubscription("Failed to obtain API key (403): ...".into());
        assert!(e.hint().unwrap().contains("jpx-jquants.com"));
    }

    #[test]
    fn test_hint_none_for_csv() {
        let e = AppError::Csv(csv::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test",
        )));
        assert!(e.hint().is_none());
    }

    #[test]
    fn test_why_none_for_csv() {
        let e = AppError::Csv(csv::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test",
        )));
        assert!(e.why().is_none());
    }
}
