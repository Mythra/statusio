use hyper::client::response::Response;
use hyper::error::Error;
use std::io::Read;

/// A function to help unwrap a Hyper Client's Response, and make sure
/// it actually responds with a body, and a successful status code.
/// The general idea of this is to get the actual result of the request,
/// instead of the request object as that's all we'll end up using.
///
/// * `resp` - A Hyper Client `Result<Response, Error>`.
///
/// Returns a `Result<String, u16>` Where the result is the response body as a string,
/// and the error is either the HTTP Status code or one a few unique errors:
///
/// Error Code `1`: The actual response object had an error.
/// Error Code `2`: We failed to read the result to a string, but the request was successful.
///
/// # Examples
///
/// You can run the get_response_body method like so with a normal request:
///
/// ```rust,no_run
/// extern crate hyper;
/// extern crate statusio;
/// let client = hyper::client::Client::new();
/// let resp = statusio::helpers::get_response_body(client.get("https://google.com").send());
/// ```
///
/// If the underlying hyper request has error'd out it will return an error code of "1":
///
/// ```rust,no_run
/// extern crate hyper;
/// extern crate statusio;
/// let client = hyper::client::Client::new();
/// let resp = statusio::helpers::get_response_body(client.get("hftp://google.com").send());
/// assert!(resp.is_err());
/// assert!(resp.err().unwrap() == 1);
/// ```
pub fn get_response_body(resp: Result<Response, Error>) -> Result<String, u16> {
    if resp.is_err() {
        return Err(1)
    }
    let mut resp = resp.unwrap();

    let status: u16 = resp.status.to_u16();

    if status < 200 || status >= 300 {
        return Err(status)
    }

    let mut resp_body = String::new();
    let read_result = resp.read_to_string(&mut resp_body);
    if read_result.is_err() {
        return Err(2)
    }

    Ok(resp_body)
}
