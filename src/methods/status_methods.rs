use ::helpers;
use hyper::method::Method;
use ::StatusIoClient;
use rustc_serialize::json;
use ::types::status;

pub fn status_summary(statuspage_id: String, client: &StatusIoClient) -> Result<status::StatusSummaryResponse, u16> {
    let resp = helpers::get_response_body(
        client.make_request(format!("status/summary/{}", &statuspage_id), Method::Get, None)
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<status::StatusSummaryResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}
