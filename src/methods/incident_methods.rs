use ::helpers;
use hyper::method::Method;
use ::StatusIoClient;
use rustc_serialize::json;
use ::types::incidents;

pub fn list_incidents(statuspage_id: String, client: &StatusIoClient) -> Result<incidents::IncidentListResponse, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("incident/list/{}", &statuspage_id), Method::Get, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<incidents::IncidentListResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn incidents_message(statuspage_id: String, message_id: String, client: &StatusIoClient) -> Result<incidents::IncidentMessageResponse, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("incident/message/{}/{}", statuspage_id, message_id), Method::Get, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<incidents::IncidentMessageResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn create_incident(incident: incidents::Incident, client: &StatusIoClient) -> Result<incidents::IncidentCreateResponse, u16> {
    let serialized_incident = json::encode(&incident).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("incident/create".to_string(), Method::Post, Some(serialized_incident))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<incidents::IncidentCreateResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn update_incident(incident: incidents::Incident, client: &StatusIoClient) -> Result<incidents::IncidentUpdateResponse, u16> {
    let serialized_incident = json::encode(&incident).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("incident/update".to_string(), Method::Post, Some(serialized_incident))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<incidents::IncidentUpdateResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn resolve_incident(incident: incidents::Incident, client: &StatusIoClient) -> Result<incidents::IncidentResolveResponse, u16> {
    let serialized_incident = json::encode(&incident).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("incident/resolve".to_string(), Method::Post, Some(serialized_incident))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<incidents::IncidentResolveResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn delete_incident(incident: incidents::Incident, client: &StatusIoClient) -> Result<incidents::IncidentDeleteResponse, u16> {
    let serialized_incident = json::encode(&incident).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("incident/delete".to_string(), Method::Post, Some(serialized_incident))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<incidents::IncidentDeleteResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}
