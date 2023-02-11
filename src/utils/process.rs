use crate::errors::Error;

use crate::utils::pid_file::{read_pidfile, write_pidfile};
use std::process::Command;
use sysinfo::SystemExt;

use nix::errno::Errno::*;
use nix::sys::signal::Signal;
use nix::unistd::Pid;

#[derive(Clone, Copy)]
pub enum ProcessState {
    // Thread is running normally.
    Running,
    // Stopped.
    Stopped,
}

pub struct Process<'b> {
    command: &'b mut Command,
    pidfile: String,
    pid: i32,
    state: ProcessState,
}

impl<'a> Process<'a> {
    pub fn new(cmd: &'a mut Command, pidfile: &str) -> Self {
        // initialize attributes
        let mut pid: i32 = 0;
        let mut state = ProcessState::Stopped;

        pid = read_pidfile(pidfile);

        if pid != 0 {
            let sys = sysinfo::System::new_all();

            match sys.process(sysinfo::Pid::from(pid)) {
                Some(_) => {
                    state = ProcessState::Running;
                }
                None => {
                    // unknown
                }
            }
        }
        return Process {
            command: cmd,
            pidfile: String::from(pidfile),
            pid: pid,
            state: state,
        };
    }

    pub fn start(&mut self) -> Result<(), Error> {
        if let Ok(child) = self.command.spawn() {
            let pid = child.id();
            write_pidfile(&self.pidfile, pid.to_string().as_str());
        }

        Ok(())
    }

    pub fn restart(&mut self, v2ray_binary: &str) -> Result<(), Error> {
        self.stop();
        self.start();
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        let result = nix::sys::signal::kill(Pid::from_raw(self.pid), Signal::SIGTERM);
        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                ESRCH => Ok(()),
                _ => {
                    panic!("stop v2ray process with err: {}", err)
                }
            },
        }
    }

    pub fn state(&self) -> ProcessState {
        return self.state;
    }

    pub fn pid(&self) -> i32 {
        return self.pid;
    }

    pub fn exist(&self) -> bool {
        let sys = sysinfo::System::new_all();
        match sys.process(sysinfo::Pid::from(self.pid)) {
            Some(_) => true,
            None => false,
        }
    }
}
