mod error;
mod utils;
mod script;
mod args;
#[cfg(test)]
mod tests;

cfg_if::cfg_if! {
    if #[cfg(feature = "async_jobs")] {
        mod async_jobs;
        use async_jobs as jobs;
    } else if #[cfg(feature = "threaded_jobs")] {
        mod thread_jobs;
        use thread_jobs as job;
    } else {
        mod bare_jobs;
        use bare_jobs as job;
    }
}

use error::Result;

fn main() {
    use std::process::exit;

    if let Err(e) = drag() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn drag() -> Result<()> {
    let args: args::Args = args::get_args()?;
    Ok(())
}