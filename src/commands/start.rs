use crate::context::Context;
use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::utils::process::Process;

use std::process::Command;

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    // start v2ray core
    let result = ctx.v2ray_process.start();
    if result.is_err() {
        let err = result.unwrap_err();
        return Err(Error {
            kind: ErrorKind::Base64DecodeError,
            message: format!("start v2ray core err: {}", err),
        });
    }

    // start worker
    let result = ctx.worker_process.start();
    if result.is_err() {
        let err = result.unwrap_err();
        return Err(Error {
            kind: ErrorKind::Base64DecodeError,
            message: format!("start v2up worker err: {}", err),
        });
    }

    // set pac
    Command::new("networksetup")
        .args([
            "-setautoproxyurl",
            "Wi-Fi",
            "http://127.0.0.1:8000/pac/proxy.js",
        ])
        .output()
        .expect("failed to enable pac");

    Ok(())
}
