use crate::errors::Error;
use crate::context::Context;
use crate::v2ray::process::{ProcessState};

use termion::{color, style};

pub fn exec(ctx: &Context) -> Result<(), Error> {
    match ctx.process.state() {
        ProcessState::Running => {
            println!("Status: {}Running{}", color::Fg(color::Green), style::Reset);
        },
        ProcessState::Stopped => {
            println!("Status: {}Stopped{}", color::Fg(color::Red), style::Reset);
        }
    }
    println!("PID: {}", ctx.process.pid());
    println!("Server: {}", ctx.config.outbounds[0].settings.vnext[0].address);

    Ok(())
}