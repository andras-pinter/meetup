mod auth;
mod ffi;
mod pam;
mod pam_result;

use auth::{AuthResult, Authenticator};
use ffi::{PamArgs, PamFlag, PamResultCode};
use pam::PamHandle;
use pam_result::PamResult;

#[no_mangle]
pub extern "C" fn pam_sm_setcred(
    _pamh: &PamHandle,
    _args: PamArgs,
    _flags: PamFlag,
) -> PamResultCode {
    PamResult::Ignore as i32
}

#[no_mangle]
pub extern "C" fn pam_sm_acct_mgmt(
    _pamh: &PamHandle,
    _args: PamArgs,
    _flags: PamFlag,
) -> PamResultCode {
    PamResult::Success as i32
}

#[no_mangle]
pub extern "C" fn pam_sm_authenticate(
    pamh: &PamHandle,
    _args: PamArgs,
    _flags: PamFlag,
) -> PamResultCode {
    match pamh.get_username() {
        Ok(_) => match pamh.get_password() {
            Ok(password) => match Authenticator::authenticate(password) {
                AuthResult::Ok => PamResult::Success as i32,
                _ => PamResult::AuthError as i32,
            },
            Err(res) => res,
        },
        Err(res) => res,
    }
}
