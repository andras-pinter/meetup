use serde_derive::Deserialize;

#[derive(Deserialize)]
pub(super) struct Response {
    pub(super) code: u8,
    pub(super) reason: String,
}
