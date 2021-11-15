use crate::ffi;
use crate::PamResultCode;

pub enum PamItem {}

pub enum PamHandle {}

type PamResultT<T> = Result<T, PamResultCode>;

impl PamHandle {
    pub fn get_username(&self) -> PamResultT<String> {
        ffi::get_username(self, "username: ")
    }

    pub fn get_password(&self) -> PamResultT<String> {
        ffi::get_authtok(self, "password: ")
    }
}
