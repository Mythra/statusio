use ::types::{maintenance, shared};

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ComponentStatus {
    pub statuspage_id: String,
    pub components: Vec<String>,
    pub containers: Vec<String>,
    pub details: String,
    pub current_status: i32,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LocationGeo {
    pub query_type: String,
    pub name: String,
    pub description: String,
    pub region: String,
    pub country: String,
    pub address: String,
    pub host: String,
    pub coords: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ComponentContainers {
    pub _id: String,
    pub name: String,
    pub __v: i32,
    pub location_geo: LocationGeo,
    pub location: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ComponentListResult {
    pub _id: String,
    pub statuspage: String,
    pub hook_key: String,
    pub containers: Vec<ComponentContainers>,
    pub history: maintenance::History,
    pub name: String,
    pub __v: i32,
    pub position: i32,
    pub description: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ComponentListResponse {
    pub status: shared::Status,
    pub result: Option<ComponentListResult>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ComponentUpdateResponse {
    pub status: shared::Status,
    pub result: Option<bool>,
}
