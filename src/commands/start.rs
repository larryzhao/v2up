use crate::errors::Error;
use crate::errors::kind::ErrorKind;
use crate::context::Context;

use fork::{daemon, Fork};
use std::process::Command;

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    // start v2ray core
    let result = ctx.process.start();
    if result.is_err() {
        let err = result.unwrap_err();
        return Err(Error {
            kind: ErrorKind::Base64DecodeError,
            message: format!("start v2ray core err: {}", err)
        })
    }

    // start worker
    if let Ok(Fork::Child) = daemon(false, false) {
        Command::new("cargo")
            .args(["run", "--", "work"])
            .current_dir("/Users/larry/Gallows/github.com/larryzhao/v2up")
            .output()
            .expect("failed to execute process");
    }

    Ok(())
}