#[repr(C)]
pub enum PamResult {
    Success = 0,
    BufError = 5,
    AuthError = 7,
    ConvError = 19,
    Ignore = 25,
}
