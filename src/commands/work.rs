use crate::context::Context;
use crate::errors::kind::ErrorKind::GetCurrentProcessIDError;
use crate::errors::Error;
use crate::server;
use crate::utils::pid_file::{read_pidfile, write_pidfile};

use libc::c_int;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use std::thread;

use sysinfo::{get_current_pid, SystemExt};

pub fn exec(ctx: &Context) -> Result<(), Error> {
    // check if worker already runing
    if worker_already_running() {
        return Ok(());
    }

    // write pid file
    match get_current_pid() {
        Ok(pid) => {
            write_pidfile(
                ctx.dir.filepath("worker.pid").as_str(),
                i32::from(pid).to_string().as_str(),
            );
        }
        Err(err_message) => {
            return Err(Error {
                kind: GetCurrentProcessIDError,
                message: format!("get current process id err: {}", err_message),
            })
        }
    }

    // start server
    let handler = thread::spawn(|| {
        server::run();
    });

    const SIGNALS: &[c_int] = &[
        SIGTERM, SIGQUIT, SIGINT, SIGTSTP, SIGWINCH, SIGHUP, SIGCHLD, SIGCONT,
    ];
    let mut sigs = Signals::new(SIGNALS).unwrap();
    for _signal in &mut sigs {
        handler.join();
        break;
    }

    Ok(())
}

/// Finds out if v2up worker process is running or not
///
/// check two things:
/// 1. if the process id in .v2up/worker.pid exists
/// 2. check if url is correctly responding
fn worker_already_running() -> bool {
    let pid = read_pidfile("/Users/larry/.v2up/worker.pid");
    if pid == 0 {
        return false;
    }

    let result = reqwest::blocking::get("http://127.0.0.1:8000/pac/proxy.js");
    if result.is_err() {
        return false;
    }

    let resp = result.unwrap();
    if !resp.status().is_success() {
        return false;
    }

    let sys = sysinfo::System::new_all();
    match sys.process(sysinfo::Pid::from(pid)) {
        Some(_) => true,
        None => false,
    }
}
