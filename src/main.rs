use getopts::Options;
use libc::{c_char, c_int, c_uint, RENAME_EXCHANGE};
use std::{env, ffi::CString, fs::File, os::unix::io::AsRawFd, process::exit};

extern "C" {
    pub fn renameat2(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_uint,
    ) -> c_int;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] SOURCE DESTINATION", program);
    print!(
        "{} (v{})\n{}\n\n{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_HOMEPAGE"),
        opts.usage(&brief)
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("v", "verbose", "display the files swapped when successful");
    opts.optflag("V", "version", "display the version and exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f);
            exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("V") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let input: Vec<&str> = if matches.free.is_empty() || matches.free.len() != 2 {
        print_usage(&program, opts);
        exit(1);
    } else {
        matches.free.iter().take(2).map(|s| s.as_str()).collect()
    };

    let from = CString::new(input[0]).expect("bad source path");
    let to = CString::new(input[1]).expect("bad destination path");

    // Use the fd of the current directory to emulate the behaviour of rename(2)
    let cdir = env::current_dir().expect("Failed to get at current dir");
    let fdir = File::open(cdir).expect("Failed to open current dir");
    let fd = fdir.as_raw_fd();

    let ret = unsafe {
        renameat2(
            fd,
            from.as_ptr(),
            fd,
            to.as_ptr(),
            RENAME_EXCHANGE as c_uint,
        )
    };
    if ret != 0 {
        eprintln!("{}", std::io::Error::last_os_error());
        exit(1);
    } else if matches.opt_present("v") {
        println!("{} <-> {}", input[0], input[1]);
    }
}
