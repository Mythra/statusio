use chrono::*;
use ::types::{maintenance, shared};

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Incident {
    pub statuspage_id: String,
    pub incident_id: Option<String>,
    pub components: Option<Vec<String>>,
    pub containers: Option<Vec<String>>,
    pub incident_name: Option<String>,
    pub incident_details: String,
    pub notify_email: i32,
    pub notify_sms: i32,
    pub notify_webhook: i32,
    pub social: i32,
    pub irc: Option<i32>,
    pub hipchat: Option<i32>,
    pub slack: Option<i32>,
    pub current_status: Option<i32>,
    pub current_state: Option<i32>,
    pub all_infrastructure_affected: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentMessages {
    pub details: String,
    pub source: String,
    pub state: i32,
    pub status: i32,
    pub statuspage: String,
    pub incident: String,
    pub ip: String,
    pub _id: String,
    pub __v: i32,
    pub datetime: DateTime<UTC>,
    pub containers: Vec<String>,
    pub components: Vec<String>
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentResponse {
    pub __v: i32,
    pub _id: String,
    pub components_affected: Vec<maintenance::ComponentsAffected>,
    pub containers_affected: Vec<maintenance::ContainersAffected>,
    pub datetime_open: DateTime<UTC>,
    pub kind: String,
    pub messages: Vec<IncidentMessages>,
    pub name: String,
    pub statuspage: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentListResult {
    pub active_incidents: Vec<IncidentResponse>,
    pub resolved_incidents: Vec<IncidentResponse>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentListResponse {
    pub status: shared::Status,
    pub result: Option<IncidentListResult>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentMessageResult {
    pub user_email: String,
    pub user_full_name: String,
    pub details: String,
    pub source: String,
    pub social: String,
    pub state: i32,
    pub status: i32,
    pub statuspage: String,
    pub incident: String,
    pub ip: String,
    pub _id: String,
    pub __v: i32,
    pub datetime: DateTime<UTC>,
    pub containers: Vec<String>,
    pub components: Vec<String>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentMessageResponse {
    pub status: shared::Status,
    pub result: Option<IncidentMessageResult>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentCreateResponse {
    pub status: shared::Status,
    pub result: Option<String>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentUpdateResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentResolveResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct IncidentDeleteResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}
