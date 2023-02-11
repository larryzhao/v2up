use crate::context::Context;
use crate::errors::Error;

use std::process::Command;

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    // start v2ray core
    ctx.v2ray_process
        .start()
        .expect("err starting v2ray core process");

    // start worker
    ctx.worker_process
        .start()
        .expect("err starting v2up worker process");

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
