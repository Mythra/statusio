use ::helpers;
use hyper::method::Method;
use ::StatusIoClient;
use rustc_serialize::json;
use ::types::components;

pub fn list_components(statuspage_id: String, client: &StatusIoClient) -> Result<components::ComponentListResult, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("component/list/{}", &statuspage_id), Method::Get, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<components::ComponentListResult, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn update_component(status: components::ComponentStatus, client: &StatusIoClient) -> Result<components::ComponentUpdateResponse, u16> {
    let serialized_status = json::encode(&status).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("component/status/update".to_string(), Method::Post, Some(serialized_status))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<components::ComponentUpdateResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}
