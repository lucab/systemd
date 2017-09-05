use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn help(short_name: *const c_char) {
    let name = unsafe { std::ffi::CStr::from_ptr(short_name) };

    let out = format!(
r#"{} [OPTIONS...]
Configures basic settings of the system.

  -h --help                    Show this help
     --version                 Show package version
     --root=PATH               Operate on an alternate filesystem root
     --locale=LOCALE           Set primary locale (LANG=)
     --locale-messages=LOCALE  Set message locale (LC_MESSAGES=)
     --timezone=TIMEZONE       Set timezone
     --hostname=NAME           Set host name
     --machine-ID=ID           Set machine ID
     --root-password=PASSWORD  Set root password
     --root-password-file=FILE Set root password from file
     --prompt-locale           Prompt the user for locale settings
     --prompt-timezone         Prompt the user for timezone
     --prompt-hostname         Prompt the user for hostname
     --prompt-root-password    Prompt the user for root password
     --prompt                  Prompt for all of the above
     --copy-locale             Copy locale from host
     --copy-timezone           Copy timezone from host
     --copy-root-password      Copy root password from host
     --copy                    Copy locale, timezone, root password
     --setup-machine-id        Generate a new random machine ID"#,
        name.to_string_lossy());
    println!("{}", out);
}
