use chrono::*;
use ::types::shared;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Metric {
    pub statuspage_id: String,
    pub metric_id: String,
    pub day_avg: f64,
    pub day_start: i64,
    pub day_dates: Vec<DateTime<UTC>>,
    pub day_values: Vec<f64>,
    pub week_avg: f64,
    pub week_start: i64,
    pub week_dates: Vec<DateTime<UTC>>,
    pub week_values: Vec<f64>,
    pub month_avg: f64,
    pub month_start: i64,
    pub month_dates: Vec<DateTime<UTC>>,
    pub month_values: Vec<f64>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MetricUpdateResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}
