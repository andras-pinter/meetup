mod req;
mod resp;
mod result;

pub(crate) use result::AuthResult;

pub(crate) struct Authenticator {}

impl Authenticator {
    const AUTHENTICATION_URL: &'static str = "http://auth-service:8080";

    pub(crate) fn authenticate(password: String) -> AuthResult {
        ureq::get(Self::AUTHENTICATION_URL)
            .send_json(req::Request { password }.into())
            .ok()
            .and_then(|resp| resp.into_json::<resp::Response>().ok())
            .map(|resp| match resp.code {
                0 => AuthResult::Ok,
                _ => AuthResult::Error(resp.reason),
            })
            .unwrap_or_default()
    }
}
