pub(crate) enum AuthResult {
    Ok,
    Error(String),
    UnknownError,
}

impl Default for AuthResult {
    fn default() -> Self {
        AuthResult::UnknownError
    }
}
