use crate::context::Context;
use crate::errors::Error;

use std::process::Command;

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    // stop v2ray
    ctx.v2ray_process
        .stop()
        .expect("err stoping v2ray core process");

    // stop v2up worker
    ctx.worker_process
        .stop()
        .expect("err stoping v2up worker process");

    // remove pac
    Command::new("networksetup")
        .args(["-setautoproxystate", "Wi-Fi", "off"])
        .output()
        .expect("failed to disable pac");

    Ok(())
}
