use ::helpers;
use hyper::method::Method;
use ::StatusIoClient;
use rustc_serialize::json;
use ::types::metric;

pub fn update_metric(metric: metric::Metric, client: &StatusIoClient) -> Result<metric::MetricUpdateResponse, u16> {
    let serialized_metric = json::encode(&metric).unwrap();
    let resp = helpers::get_response_body(
        client.make_request("metric/update".to_string(), Method::Post, Some(serialized_metric))
    );

    if resp.is_err() {
        return Err(resp.err().unwrap())
    }
    let resp = resp.unwrap();

    let decoded: Result<metric::MetricUpdateResponse, json::DecoderError> = json::decode(&resp);
    if decoded.is_err() {
        return Err(999)
    }
    let decoded = decoded.unwrap();

    Ok(decoded)
}
