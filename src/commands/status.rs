use crate::context::Context;
use crate::errors::Error;
use crate::utils::process::ProcessState;

use termion::{color, style};

pub fn exec(ctx: &Context) -> Result<(), Error> {
    match ctx.v2ray_process.state() {
        ProcessState::Running => {
            println!("Status: {}Running{}", color::Fg(color::Green), style::Reset);
        }
        ProcessState::Stopped => {
            println!("Status: {}Stopped{}", color::Fg(color::Red), style::Reset);
        }
    }
    println!("PID: {}", ctx.v2ray_process.pid());
    println!("Server: {}", ctx.config.outbounds[0].settings.address());

    Ok(())
}
