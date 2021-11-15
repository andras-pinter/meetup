use crate::pam::{PamHandle, PamItem};
use crate::pam_result::PamResult;

pub type PamResultCode = libc::c_int;
pub type PamFlag = libc::c_uint;
pub type PamArgs = *const libc::c_char;
type PamItemType = libc::c_int;

const PAM_AUTHTOK: PamItemType = 6;

pub(crate) fn get_username(
    pamh: &PamHandle,
    prompt: &'static str,
) -> Result<String, PamResultCode> {
    let prompt = std::ffi::CString::new(prompt).expect("Invalid null byte in prompt");
    let username_ptr = std::ptr::null_mut::<libc::c_char>();

    unsafe {
        match ffipam::pam_get_user(pamh, &username_ptr, prompt.as_ptr()) {
            0 if !username_ptr.is_null() => std::ffi::CStr::from_ptr(username_ptr)
                .to_str()
                .map_err(|_| PamResult::ConvError as i32)
                .map(str::to_string),
            0 => Err(PamResult::BufError as i32),
            ret => Err(ret),
        }
    }
}

pub(crate) fn get_authtok(pamh: &PamHandle, prompt: &'static str) -> Result<String, PamResultCode> {
    let prompt = std::ffi::CString::new(prompt).expect("Invalid null byte in prompt");
    let mut item_ptr = std::ptr::null::<PamItem>();

    unsafe {
        match ffipam::pam_get_authtok(pamh, PAM_AUTHTOK, &mut item_ptr, prompt.as_ptr()) {
            0 if !item_ptr.is_null() => std::ffi::CStr::from_ptr(item_ptr.cast())
                .to_str()
                .map_err(|_| PamResult::ConvError as i32)
                .map(str::to_string),
            0 => Err(PamResult::BufError as i32),
            ret => Err(ret),
        }
    }
}

mod ffipam {
    use super::*;

    type CharArray = *const libc::c_char;
    type CharArrayMut<'a> = &'a *mut libc::c_char;

    #[link(name = "pam")]
    extern "C" {
        pub fn pam_get_user(
            pamh: *const PamHandle,
            user: CharArrayMut,
            prompt: CharArray,
        ) -> PamResultCode;
    }

    #[link(name = "pam_misc")]
    extern "C" {
        pub fn pam_get_authtok(
            pamh: *const PamHandle,
            item_type: PamItemType,
            item: &mut *const PamItem,
            prompt: CharArray,
        ) -> PamResultCode;
    }
}
