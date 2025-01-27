use chrono::Utc;
use klickhouse::Row;

#[derive(Row, Debug)]
pub struct WebServerLog {
    pub timestamp: chrono::DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub module: String,
    pub request_id: String,
    pub uri: String,
    pub method: String,
    pub status_code: i32,
    pub response_time: f64,
}
