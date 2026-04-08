use crate::config::Config;
use crate::error::AppError;
use crate::models::{
    AmBar, ApiErrorResponse, ApiResponse, Breakdown, BulkGetResponse, BulkListItem, Calendar,
    DailyBar, EarningsCalendar, FinsDetails, FinsDividend, FinsSummary, FuturesBar, IndexDailyBar,
    InvestorType, MarginAlert, MarginInterest, MinuteBar, Options225Bar, OptionsBar, ShortRatio,
    ShortSaleReport, StockMaster, TopixDailyBar,
};
use reqwest::Client;

const HEADER_API_KEY: &str = "x-api-key";

pub struct JQuantsClient {
    client: Client,
    api_key: String,
    base_url: String,
}

fn build_params<'a>(entries: &[(&'a str, Option<&'a str>)]) -> Vec<(&'a str, &'a str)> {
    entries
        .iter()
        .filter_map(|(k, v)| v.map(|val| (*k, val)))
        .collect()
}

// endpoint パラメータの先頭に "/" を補完する
fn normalize_endpoint(endpoint: Option<&str>) -> Option<String> {
    endpoint.map(|e| {
        if e.starts_with('/') {
            e.to_owned()
        } else {
            format!("/{e}")
        }
    })
}

impl JQuantsClient {
    pub fn new(config: Config) -> Self {
        JQuantsClient {
            client: Client::new(),
            api_key: config.api_key,
            base_url: config.base_url,
        }
    }

    pub fn http_client(&self) -> &Client {
        &self.client
    }

    async fn parse_api_error(response: reqwest::Response, status: u16) -> AppError {
        let error_body: ApiErrorResponse = response.json().await.unwrap_or(ApiErrorResponse {
            message: format!("HTTP {}", status),
        });
        AppError::Api {
            status,
            message: error_body.message,
        }
    }

    async fn fetch_paginated<T>(
        &self,
        path: &str,
        params: Vec<(&str, &str)>,
    ) -> Result<Vec<T>, AppError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut all_results: Vec<T> = Vec::new();
        let mut pagination_key: Option<String> = None;
        let url = format!("{}{}", self.base_url, path);

        loop {
            let mut request = self
                .client
                .get(&url)
                .header(HEADER_API_KEY, &self.api_key)
                .query(&params);
            if let Some(ref pk) = pagination_key {
                request = request.query(&[("pagination_key", pk.as_str())]);
            }
            let response = request.send().await?;

            let status = response.status().as_u16();
            if status == 200 {
                let text = response.text().await?;
                let body: ApiResponse<T> =
                    serde_json::from_str(&text).map_err(|e| AppError::Decode {
                        source: e,
                        body: text.chars().take(500).collect(),
                    })?;
                all_results.extend(body.data);
                match body.pagination_key {
                    Some(key) => pagination_key = Some(key),
                    None => break,
                }
            } else if status == 210 {
                break;
            } else {
                return Err(Self::parse_api_error(response, status).await);
            }
        }

        Ok(all_results)
    }

    pub async fn get_stock_master(
        &self,
        code: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<StockMaster>, AppError> {
        let params = build_params(&[("code", code), ("date", date)]);
        self.fetch_paginated("/equities/master", params).await
    }

    pub async fn get_am_bars(&self, code: Option<&str>) -> Result<Vec<AmBar>, AppError> {
        let params = build_params(&[("code", code)]);
        self.fetch_paginated("/equities/bars/daily/am", params)
            .await
    }

    pub async fn get_breakdown(
        &self,
        code: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<Breakdown>, AppError> {
        let params = build_params(&[("code", code), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/markets/breakdown", params).await
    }

    pub async fn get_minute_bars(
        &self,
        code: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<MinuteBar>, AppError> {
        let params = build_params(&[("code", code), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/equities/bars/minute", params).await
    }

    pub async fn get_daily_bars(
        &self,
        code: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<DailyBar>, AppError> {
        let params = build_params(&[("code", code), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/equities/bars/daily", params).await
    }

    pub async fn get_bulk_list(
        &self,
        endpoint: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<BulkListItem>, AppError> {
        let endpoint_owned = normalize_endpoint(endpoint);
        let params = build_params(&[
            ("endpoint", endpoint_owned.as_deref()),
            ("date", date),
            ("from", from),
            ("to", to),
        ]);
        self.fetch_paginated("/bulk/list", params).await
    }

    pub async fn get_calendar(
        &self,
        hol_div: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<Calendar>, AppError> {
        let params = build_params(&[("holidayDivision", hol_div), ("from", from), ("to", to)]);
        self.fetch_paginated("/markets/calendar", params).await
    }

    pub async fn get_options_225_bars(
        &self,
        date: Option<&str>,
    ) -> Result<Vec<Options225Bar>, AppError> {
        let params = build_params(&[("date", date)]);
        self.fetch_paginated("/derivatives/bars/daily/options/225", params)
            .await
    }

    pub async fn get_futures_bars(
        &self,
        category: Option<&str>,
        date: Option<&str>,
        contract_flag: Option<&str>,
    ) -> Result<Vec<FuturesBar>, AppError> {
        let params = build_params(&[
            ("category", category),
            ("date", date),
            ("contractFlag", contract_flag),
        ]);
        self.fetch_paginated("/derivatives/bars/daily/futures", params)
            .await
    }

    pub async fn get_options_bars(
        &self,
        category: Option<&str>,
        code: Option<&str>,
        date: Option<&str>,
        contract_flag: Option<&str>,
    ) -> Result<Vec<OptionsBar>, AppError> {
        let params = build_params(&[
            ("category", category),
            ("code", code),
            ("date", date),
            ("contractFlag", contract_flag),
        ]);
        self.fetch_paginated("/derivatives/bars/daily/options", params)
            .await
    }

    pub async fn get_earnings_calendar(&self) -> Result<Vec<EarningsCalendar>, AppError> {
        self.fetch_paginated("/equities/earnings-calendar", vec![])
            .await
    }

    pub async fn get_fins_details(
        &self,
        code: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<FinsDetails>, AppError> {
        let params = build_params(&[("code", code), ("date", date)]);
        self.fetch_paginated("/fins/details", params).await
    }

    pub async fn get_fins_dividend(
        &self,
        code: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<FinsDividend>, AppError> {
        let params = build_params(&[("code", code), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/fins/dividend", params).await
    }

    pub async fn get_fins_summary(
        &self,
        code: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<FinsSummary>, AppError> {
        let params = build_params(&[("code", code), ("date", date)]);
        self.fetch_paginated("/fins/summary", params).await
    }

    pub async fn get_topix_daily_bars(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<TopixDailyBar>, AppError> {
        let params = build_params(&[("from", from), ("to", to)]);
        self.fetch_paginated("/indices/bars/daily/topix", params)
            .await
    }

    pub async fn get_index_daily_bars(
        &self,
        code: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IndexDailyBar>, AppError> {
        let params = build_params(&[("code", code), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/indices/bars/daily", params).await
    }

    pub async fn get_investor_types(
        &self,
        section: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<InvestorType>, AppError> {
        let params = build_params(&[("section", section), ("from", from), ("to", to)]);
        self.fetch_paginated("/equities/investor-types", params)
            .await
    }

    pub async fn get_margin_alert(
        &self,
        code: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<MarginAlert>, AppError> {
        let params = build_params(&[("code", code), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/markets/margin-alert", params).await
    }

    pub async fn get_margin_interest(
        &self,
        code: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<MarginInterest>, AppError> {
        let params = build_params(&[("code", code), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/markets/margin-interest", params)
            .await
    }

    pub async fn get_short_ratio(
        &self,
        s33: Option<&str>,
        date: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<ShortRatio>, AppError> {
        let params = build_params(&[("s33", s33), ("date", date), ("from", from), ("to", to)]);
        self.fetch_paginated("/markets/short-ratio", params).await
    }

    pub async fn get_short_sale_report(
        &self,
        code: Option<&str>,
        disc_date: Option<&str>,
        disc_date_from: Option<&str>,
        disc_date_to: Option<&str>,
        calc_date: Option<&str>,
    ) -> Result<Vec<ShortSaleReport>, AppError> {
        let params = build_params(&[
            ("code", code),
            ("disc_date", disc_date),
            ("disc_date_from", disc_date_from),
            ("disc_date_to", disc_date_to),
            ("calc_date", calc_date),
        ]);
        self.fetch_paginated("/markets/short-sale-report", params)
            .await
    }

    pub async fn get_bulk(
        &self,
        key: Option<&str>,
        endpoint: Option<&str>,
        date: Option<&str>,
    ) -> Result<String, AppError> {
        let endpoint_owned = normalize_endpoint(endpoint);
        let params = build_params(&[
            ("key", key),
            ("endpoint", endpoint_owned.as_deref()),
            ("date", date),
        ]);
        let url = format!("{}/bulk/get", self.base_url);

        let response = self
            .client
            .get(&url)
            .header(HEADER_API_KEY, &self.api_key)
            .query(&params)
            .send()
            .await?;

        let status = response.status().as_u16();
        if status == 200 {
            let text = response.text().await?;
            let body: BulkGetResponse =
                serde_json::from_str(&text).map_err(|e| AppError::Decode {
                    source: e,
                    body: text.chars().take(500).collect(),
                })?;
            Ok(body.url)
        } else {
            Err(Self::parse_api_error(response, status).await)
        }
    }
}
