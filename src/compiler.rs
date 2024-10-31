use std::fs::write;
use std::io::Error;
use std::process::{Command, Output};

pub(crate) fn compile_code(code: &str) -> bool {
    let filename: &str = "temp.c";

    if let Err(e) = write(filename, code) {
        eprintln!("Error writing to file: {}", e);
        return false;
    }

    let compile_result: Result<Output, Error> = Command::new("gcc")
        .arg(filename)
        .arg("-o")
        .arg("temp_exec")
        .output();

    if let Ok(output) = compile_result {
        return output.status.success();
    }
    false
}

pub(crate) fn run_code() -> bool {
    let run_result: Result<Output, Error> = Command::new("./temp_exec").output();
    run_result.is_ok()
}
