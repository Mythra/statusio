#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Status {
    pub error: String,
    pub message: String,
}
