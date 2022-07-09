use crate::errors::Error;
use crate::context::Context;

use termion::{color, style};

pub fn exec(ctx: &Context) -> Result<(), Error> {
    println!("{}Red", color::Fg(color::Red));
    Ok(())
}