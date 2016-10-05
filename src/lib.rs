extern crate chrono;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate mime;
extern crate rustc_serialize;

pub mod helpers;
pub mod methods;
pub mod types;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Error;
use hyper::header::ContentType;
use hyper::method::Method;
use mime::Mime;
use std::str::FromStr;

header! {
    /// A Hyper Header mapping for the "x-api-id" header which is used for authentication
    /// inside of `status.io`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate statusio;
    /// let header = statusio::XApiId("MY API ID.".to_string());
    /// ```
    (XApiId, "x-api-id") => [String]
}
header! {
    /// A Hyper header mapping for the "x-api-key" header which is used for authentication
    /// inside of `status.io`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate statusio;
    /// let header = statusio::XApiKey("MY API Key.".to_string());
    /// ```
    (XApiKey, "x-api-key") => [String]
}

lazy_static! {
    static ref API_ENDPOINT: String = "https://api.status.io/v2/".to_owned();
}

/// The actual StatusIoClient Struct. Basically automatically assigns the auth
/// headers to every single request you make so you don't have too. Making Status.io
/// Great again one step at a time. All the items inside here are private since there
/// shouldn't be a reason you need to access these.
#[derive(Debug)]
pub struct StatusIoClient {
    api_id: String,
    api_key: String,
    client: Client,
    base_url: String,
}

impl StatusIoClient {
    /// Constructs a new Status.io Client for your personal use. Helps actually create
    /// the hyper client as well as the base url for you.
    ///
    /// * `api_id` - The API ID used to authenticate with status.io
    /// * `api_key` - The API Key used to authenticate with status.io
    /// * `base_url` - An optional Base URL. If `None` will use the status.io default of: `https://api.status.io/v2/`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate statusio;
    /// let client = statusio::StatusIoClient::new("API Key".to_string(), "API ID".to_string(), None);
    /// let client = statusio::StatusIoClient::new("API Key".to_string(), "API ID".to_string(), Some("https://custom_base_url.com".to_string()));
    /// ```
    pub fn new(api_id: String, api_key: String, base_url: Option<String>) -> StatusIoClient {
        let url: String;
        if base_url.is_none() {
            url = API_ENDPOINT.clone();
        } else {
            url = base_url.unwrap();
        }

        StatusIoClient {
            api_id: api_id,
            api_key: api_key,
            client: Client::new(),
            base_url: url,
        }
    }

    /// An internal method to determine if a particular HTTP Method can have an
    /// http body. So we don't attend to send an invalid request on to hyper.
    fn method_can_have_body(&self, method: &Method) -> bool {
        match method {
            &Method::Connect | &Method::Get | &Method::Options | &Method::Trace => false,
            _ => true
        }
    }

    /// Makes an HTTP request through hyper to status io. This will automatically append
    /// three headers to your request: "x-api-id", "x-api-key", and "Content-Type" (as application/json).
    ///
    /// * `path` - The path to be appended onto the base path.
    /// * `method` - The HTTP Method to use.
    /// * `body` - An optional body to append to the request.
    ///
    /// Returns a hyper response object.
    ///
    /// # Examples
    ///
    /// Making a simple GET Request:
    ///
    /// ```rust,no_run
    /// extern crate hyper;
    /// extern crate statusio;
    /// let client = statusio::StatusIoClient::new("API Key".to_string(), "API ID".to_string(), None);
    /// client.make_request("incident/list/<status_page_id>".to_string(), hyper::method::Method::Get, None);
    /// ```
    ///
    /// Making a simple POST Request with a body:
    ///
    /// ```rust,no_run
    /// extern crate hyper;
    /// extern crate statusio;
    /// let client = statusio::StatusIoClient::new("API Key".to_string(), "API ID".to_string(), None);
    /// client.make_request("incident/list/<status_page_id>".to_string(), hyper::method::Method::Post, Some("{}".to_string()));
    /// ```
    pub fn make_request(&self, path: String, method: Method, body: Option<String>) -> Result<Response, Error> {
        let can_have_body = self.method_can_have_body(&method);
        let has_body = body.is_some();
        let body_result = body.unwrap_or("".to_string());

        let mut request = self.client.request(method, &format!("{}{}", self.base_url, path));

        if has_body && can_have_body {
            request = request.body(&body_result);
        }

        request.header(XApiId(self.api_id.clone()))
            .header(XApiKey(self.api_key.clone()))
            .header(ContentType(Mime::from_str(&"application/json".to_string()).unwrap()))
            .send()
    }
}
