use std::os::raw::{c_char, c_int};
use std::ptr::null;
use std::ffi::CStr;
use std::os::unix::fs;

const EINVAL: c_int = 22;
const EEXIST: c_int = 17;
const EIO: c_int = 5;

#[no_mangle]
pub extern "C" fn add_alias(
    dest_in: *const c_char,
    service_in: *const c_char,
    alias_in: *const c_char,
) -> c_int {
    if service_in == null() || alias_in == null() || dest_in == null() {
        return -EINVAL;
    };

    let alias = unsafe { CStr::from_ptr(alias_in).to_string_lossy() };
    let dest = unsafe { CStr::from_ptr(dest_in).to_string_lossy() };
    let service = unsafe { CStr::from_ptr(service_in).to_string_lossy() };

    let link = String::from(dest) + "/" + &alias;
    let target = String::from(service);
    if let Err(e) = fs::symlink(target, link) {
        return match e.raw_os_error() {
            Some(EEXIST) => 0,
            Some(errno) => -errno,
            None => -EIO,
        };
    };
    0
}
