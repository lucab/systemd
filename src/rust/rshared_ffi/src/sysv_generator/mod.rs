//! Helper logic for systemd-sysv-generator(8).
use rshared::sysv_generator;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

const EINVAL: c_int = 22;
const EIO: c_int = 5;

/// FFI wrapper for [add_alias].
///
/// Returns:
///  - `1` if alias was created.
///  - `0` if alias path already existed.
///  - a negative errno code on errors.
///
/// [add_alias]: ../../rshared/sysv_generator/fn.add_alias.html
#[no_mangle]
pub extern "C" fn add_alias(
    dest_in: *const c_char,
    service_in: *const c_char,
    alias_in: *const c_char,
) -> c_int {
    if dest_in.is_null() || service_in.is_null() || alias_in.is_null() {
        return -EINVAL;
    };

    let alias = unsafe { CStr::from_ptr(alias_in) }.to_string_lossy();
    let dest = unsafe { CStr::from_ptr(dest_in) }.to_string_lossy();
    let service = unsafe { CStr::from_ptr(service_in) }.to_string_lossy();

    let r = sysv_generator::add_alias(&dest, &alias, &service).map_err(|e| e.raw_os_error());
    match r {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(Some(errno)) => -errno,
        Err(None) => -EIO,
    }
}
