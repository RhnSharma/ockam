use rand::random;
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum InfluxError {
    Authentication(String),
    Unknown,
}

impl Display for InfluxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InfluxError::Authentication(msg) => f.write_str(msg),
            _ => f.write_str("Unknown"),
        }
    }
}

impl Error for InfluxError {}

// A user's special Influx integration
pub struct InfluxClient {
    api_url: String,
    org: String,
    bucket: String,
    leased_token: String,
}

impl InfluxClient {
    pub fn new(api_url: &str, org: &str, bucket: &str, leased_token: &str) -> Self {
        InfluxClient {
            api_url: api_url.to_string(),
            org: org.to_string(),
            bucket: bucket.to_string(),
            leased_token: leased_token.to_string(),
        }
    }

    pub fn set_token(&mut self, leased_token: &str) {
        self.leased_token = leased_token.to_string();
    }

    pub async fn send_metrics(&self) -> Result<(), InfluxError> {
        let url = format!(
            "{}/api/v2/write?org={}&bucket={}&precision=s",
            self.api_url, self.org, self.bucket
        );

        let mut headers = HeaderMap::new();
        let token = format!("Token {}", self.leased_token);

        headers.insert(
            "Authorization",
            HeaderValue::from_str(token.as_str()).unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        for i in 0..10 {
            let data = random::<usize>() % 10_000;
            let metric = format!("metrics,env=test r{}={}", i, data);
            let resp = client.post(url.clone()).body(metric).send().await.unwrap();
            if resp.status().as_u16() == 401 {
                return Err(InfluxError::Authentication(
                    "Authentication failed.".to_string(),
                ));
            }
        }
        Ok(())
    }
}
