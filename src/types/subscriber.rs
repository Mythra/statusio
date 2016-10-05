use chrono::*;
use ::types::shared;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Subscriber {
    pub statuspage_id: String,
    pub subscriber_id: Option<String>,
    pub method: String,
    pub address: String,
    pub granular: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SubscriberResponse {
    pub _id: String,
    pub address: String,
    pub method: String,
    pub statuspage: String,
    pub __v: i32,
    pub granular: Vec<String>,
    pub active: bool,
    pub join_date: DateTime<UTC>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SubscriberListResult {
    pub email: Vec<SubscriberResponse>,
    pub webhook: Vec<SubscriberResponse>,
    pub sms: Vec<SubscriberResponse>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SubscriberListResponse {
    pub status: shared::Status,
    pub result: Option<SubscriberListResult>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SubscriberAddResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
    pub subscriber_id: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SubscriberUpdateResponse {
    pub status: shared::Status,
    pub result: Option<SubscriberResponse>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SubscriberRemoveResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}
