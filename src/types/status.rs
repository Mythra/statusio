use chrono::*;
use ::types::shared;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub updated: DateTime<UTC>,
    pub status: String,
    pub status_code: i32,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Status {
    pub id: String,
    pub name: String,
    pub containers: Vec<Container>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct StatusSummaryResult {
    pub status: Vec<Status>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct StatusSummaryResponse {
    pub status: shared::Status,
    pub result: StatusSummaryResult,
}
