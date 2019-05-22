use getopts::Options;
use std::{env, path::PathBuf, process::exit};

pub mod strategy;
pub use strategy::Strategy;

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

    let input: Vec<PathBuf> = if matches.free.is_empty() || matches.free.len() != 2 {
        print_usage(&program, opts);
        exit(1);
    } else {
        matches.free.iter().take(2).map(|s| s.into()).collect()
    };

    match Strategy::RenameAt2.swap(&input[0], &input[1]) {
        Ok(_) => {
            if matches.opt_present("v") {
                println!("{:?} <-> {:?}", input[0], input[1]);
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1);
        }
    }
}
