use serde_derive::Serialize;

#[derive(Serialize)]
pub(super) struct Request {
    pub(super) password: String,
}

impl Into<serde_json::Value> for Request {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Invalid JSON!")
    }
}
