//! Helper logic for systemd-sysv-generator(8).

/// Add a name alias for a given service under the destination directory.
///
/// This acts on the filesystem by introducing a symlink for the service alias,
/// and returns whether the symlink was created.
pub fn add_alias(dest: &str, alias: &str, service: &str) -> std::io::Result<bool> {
    use std::os::unix::fs;

    let linkpath = dest.to_string() + "/" + alias;
    let res = fs::symlink(service, linkpath).and(Ok(true));
    if let Err(ref errno) = res {
        if errno.kind() == std::io::ErrorKind::AlreadyExists {
            return Ok(false);
        }
    };
    res
}
