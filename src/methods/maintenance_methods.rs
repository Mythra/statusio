use ::helpers;
use hyper::method::Method;
use ::StatusIoClient;
use rustc_serialize::json;
use ::types::maintenance;

pub fn list_maintenance(statuspage_id: String, client: &StatusIoClient) -> Result<maintenance::MaintenanceListResponse, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("maintenance/list/{}", &statuspage_id), Method::Get, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<maintenance::MaintenanceListResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn message_maintenance(statuspage_id: String, message_id: String, client: &StatusIoClient) -> Result<maintenance::MaintenanceMessageResponse, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("maintenance/message/{}/{}", statuspage_id, message_id), Method::Get, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<maintenance::MaintenanceMessageResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn schedule_maintenance(task: maintenance::Maintenance, client: &StatusIoClient) -> Result<maintenance::MaintenanceScheduleResponse, u16> {
    let serialized_task = json::encode(&task).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("maintenance/schedule".to_string(), Method::Post, Some(serialized_task))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<maintenance::MaintenanceScheduleResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn start_maintenance(control: maintenance::MaintenanceContol, client: &StatusIoClient) -> Result<maintenance::MaintenanceStartResponse, u16> {
    let serialized_control = json::encode(&control).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("maintenance/start".to_string(), Method::Post, Some(serialized_control))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<maintenance::MaintenanceStartResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn update_maintenance(control: maintenance::MaintenanceContol, client: &StatusIoClient) -> Result<maintenance::MaintenanceUpdateResponse, u16> {
    let serialized_control = json::encode(&control).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("maintenance/update".to_string(), Method::Post, Some(serialized_control))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<maintenance::MaintenanceUpdateResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn finish_maintenance(control: maintenance::MaintenanceContol, client: &StatusIoClient) -> Result<maintenance::MaintenanceFinishResponse, u16> {
    let serialized_control = json::encode(&control).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("maintenance/finish".to_string(), Method::Post, Some(serialized_control))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<maintenance::MaintenanceFinishResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn delete_maintenance(control: maintenance::MaintenanceContol, client: &StatusIoClient) -> Result<maintenance::MaintenanceDeleteResponse, u16> {
    let serialized_control = json::encode(&control).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("maintenance/delete".to_string(), Method::Post, Some(serialized_control))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<maintenance::MaintenanceDeleteResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}
