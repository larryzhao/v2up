use crate::context::Context;
use crate::errors::Error;
use crate::errors::kind::ErrorKind::{GetCurrentProcessIDError};
use crate::server;
use crate::utils::pid_file::{read_pidfile, write_pidfile};

use sysinfo::{SystemExt, get_current_pid, Pid};
use fork::{daemon, Fork};
use std::process::Command;
use std::path::Path;

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    // check if worker already runing
    if worker_already_running() {
        return Ok(())
    }

    // start worker for subscriptions

    // write pid file
    match get_current_pid() {
        Ok(pid) => {
           write_pidfile("/Users/larry/.v2up/worker.pid", i32::from(pid).to_string().as_str());
        },
        Err(err_message) => {
            return Err(Error {
                kind: GetCurrentProcessIDError,
                message: format!("get current process id err: {}", err_message),
            })
        }
    }
    
    // start server
    server::run();

    Ok(())
}

fn worker_already_running() -> bool {
    let pid = read_pidfile("/Users/larry/.v2up/worker.pid");
    if pid == 0 {
        return false
    }

    let sys = sysinfo::System::new_all();

    match sys.process(sysinfo::Pid::from(pid)) {
        Some(_) => {
           true
        }
        None => {
            false
        }
    }
}