use std::io;
use std::process::ExitCode;

fn sub() -> Result<(), io::Error> {
    rs_zips2jsonl::stdin2filenames2zips2stdout()
}

fn main() -> ExitCode {
    match sub() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
