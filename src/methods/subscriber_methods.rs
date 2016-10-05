use ::helpers;
use hyper::method::Method;
use ::StatusIoClient;
use rustc_serialize::json;
use ::types::subscriber;

pub fn list_subscriber(statuspage_id: String, client: &StatusIoClient) -> Result<subscriber::SubscriberListResponse, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("subscriber/list/{}", &statuspage_id), Method::Get, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<subscriber::SubscriberListResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn add_subscriber(subscriber: subscriber::Subscriber, client: &StatusIoClient) -> Result<subscriber::SubscriberAddResponse, u16> {
    let serialized_subscriber = json::encode(&subscriber).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("subscriber/add".to_string(), Method::Post, Some(serialized_subscriber))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<subscriber::SubscriberAddResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn update_subscriber(subscriber: subscriber::Subscriber, client: &StatusIoClient) -> Result<subscriber::SubscriberUpdateResponse, u16> {
    let serialized_subscriber = json::encode(&subscriber).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("subscriber/update".to_string(), Method::Patch, Some(serialized_subscriber))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<subscriber::SubscriberUpdateResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}

pub fn remove_subscriber(statuspage_id: String, subscriber_id: String, client: &StatusIoClient) -> Result<subscriber::SubscriberRemoveResponse, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("subscriber/remove/{}/{}", &statuspage_id, &subscriber_id), Method::Delete, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<subscriber::SubscriberRemoveResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}
