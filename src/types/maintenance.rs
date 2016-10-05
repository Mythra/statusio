use chrono::*;
use ::types::shared;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Maintenance {
    pub statuspage_id: String,
    pub components: Vec<String>,
    pub containers: Vec<String>,
    pub all_infrastructure_affected: Option<i32>,
    pub automation: i32,
    pub maintenance_name: String,
    pub maintenance_details: String,
    pub date_planned_start: String,
    pub time_planned_start: String,
    pub date_planned_end: String,
    pub time_planned_end: String,
    pub maintenance_notify_now: i32,
    pub maintenance_notify_72_hr: i32,
    pub maintenance_notify_24_hr: i32,
    pub maintenance_notify_1_hr: i32,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceContol {
    pub statuspage_id: String,
    pub maintenance_id: String,
    pub maintenance_details: Option<String>,
    pub notify_email: i32,
    pub notify_sms: i32,
    pub notify_webhook: i32,
    pub social: i32,
    pub irc: Option<i32>,
    pub hipchat: Option<i32>,
    pub slack: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct History {
    pub message_id: String,
    pub _id: String,
    pub datetime: DateTime<UTC>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ContainersAffected {
    pub __v: i32,
    pub _id: String,
    pub name: String,
    pub location: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ComponentsAffected {
    pub __v: i32,
    pub _id: String,
    pub position: String,
    pub statuspage: String,
    pub history: Vec<History>,
    pub containers: Vec<String>,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Messages {
    pub details: String,
    pub source: String,
    pub state: i32,
    pub status: i32,
    pub statuspage: String,
    pub maintenance: String,
    pub ip: String,
    pub _id: String,
    pub __v: i32,
    pub datetime: DateTime<UTC>,
    pub containers: Vec<String>,
    pub components: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceResponse {
    pub __v: i32,
    pub _id: String,
    pub components_affected: Vec<ComponentsAffected>,
    pub containers_affected: Vec<ContainersAffected>,
    pub datetime_open: DateTime<UTC>,
    pub datetime_planned_end: DateTime<UTC>,
    pub datetime_planned_start: DateTime<UTC>,
    pub kind: String,
    pub messages: Vec<Messages>,
    pub name: String,
    pub statuspage: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceListResult {
    pub active_maintenances: Vec<MaintenanceResponse>,
    pub upcoming_maintenances: Vec<MaintenanceResponse>,
    pub resolved_maintenances: Vec<MaintenanceResponse>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceListResponse {
    pub status: shared::Status,
    pub result: Option<MaintenanceListResult>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceMessageResult {
    pub details: String,
    pub source: String,
    pub state: i32,
    pub statuspage: String,
    pub maintenance: String,
    pub ip: String,
    pub _id: String,
    pub __v: i32,
    pub datetime: DateTime<UTC>,
    pub containers: Vec<String>,
    pub components: Vec<String>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceMessageResponse {
    pub status: shared::Status,
    pub result: Option<MaintenanceMessageResult>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceScheduleResponse {
    pub status: shared::Status,
    pub result: Option<String>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceStartResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceUpdateResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceFinishResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MaintenanceDeleteResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}
