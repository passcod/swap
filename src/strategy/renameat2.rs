use super::Error;
use libc::{c_char, c_int, c_uint, RENAME_EXCHANGE};
use std::os::unix::ffi::OsStrExt;
use std::{env, ffi::CString, fs::File, os::unix::io::AsRawFd, path::Path};

extern "C" {
    pub fn renameat2(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_uint,
    ) -> c_int;
}

pub fn swap<F, T>(from: F, to: T) -> Result<(), Error>
where
    F: AsRef<Path>,
    T: AsRef<Path>,
{
    let from = CString::new(from.as_ref().as_os_str().as_bytes())?;
    let to = CString::new(to.as_ref().as_os_str().as_bytes())?;

    // Use the fd of the current directory to emulate the behaviour of rename(2)
    let cwd = File::open(env::current_dir()?)?.as_raw_fd();

    let ret = unsafe {
        renameat2(
            cwd,
            from.as_ptr(),
            cwd,
            to.as_ptr(),
            RENAME_EXCHANGE as c_uint,
        )
    };

    if ret != 0 {
        Err(Error::from_last_io())
    } else {
        Ok(())
    }
}
