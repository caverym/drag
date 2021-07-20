use std::ffi::OsString;
use crate::error::Result;

pub struct Args {
    jobs: Option<u8>,
    file: Option<OsString>,
}

pub fn get_args() -> Result<Args> {
    let mut pargs: pico_args::Arguments = pico_args::Arguments::from_env();

    let args: Args = Args {
        jobs: pargs.opt_value_from_str(["-j", "--jobs"])?,
        file: pargs.opt_value_from_str(["-f", "--file"])?,
    };

    Ok(args)
}

fn parse_jobs(f: &str) -> Result<u8> {
    dbg!("{}", f);
    todo!()
}
