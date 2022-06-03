use crate::utils::pid_file::write_pidfile;
use crate::errors::kind::ErrorKind;
use crate::errors::Error;

use std::fs;
use std::path::PathBuf;
use sysinfo::SystemExt;
use std::process::Command;

use nix::sys::signal::{Signal};
use nix::unistd::Pid;

enum ProcessState {
    /// Thread is running normally.
    Running,
    /// Thread is stopped.
    Stopped,
    /// Thread is waiting normally.
    Waiting,
    /// Thread is in an uninterruptible wait
    Uninterruptible,
    /// Thread is halted at a clean point.
    Halted,
    /// Unknown.
    Unknown(i32),
}

pub struct Process {
    pid: i32,
    state: ProcessState,
}

impl Process {
    pub fn new() -> Self {
        let mut pid: i32 = 0;
        let mut state = ProcessState::Stopped;

        let path: PathBuf = ["/Users/larry/.v2up/", "v2ray.pid"].iter().collect();
        let result = path.canonicalize();
        if result.is_ok() {
            let pid_file = result.unwrap();
            let pid_str = fs::read_to_string(String::from(pid_file.to_string_lossy()))
                .unwrap_or(String::from("0"));
            pid = pid_str.trim().parse().unwrap();
        }

        if pid != 0 {
            let sys = sysinfo::System::new_all();

            match sys.process(sysinfo::Pid::from(pid)) {
                Some(_) => {
                    state = ProcessState::Running;
                }
                None => {
                    // warn
                }
            }
        }
        return Process { pid, state };
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let mut command = Command::new("/usr/local/bin/v2ray");
        command.args(["-config", "/Users/larry/.v2up/v2ray.json", "&"]);

        if let Ok(child) = command.spawn() {
            let pid = child.id();
            write_pidfile("/Users/larry/.v2up/v2ray.pid", pid.to_string().as_str());
        }

        Ok(())
    }

    pub fn restart(&mut self) -> Result<(), Error> {
        self.stop();
        self.start();
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        nix::sys::signal::kill(Pid::from_raw(self.pid), Signal::SIGTERM).unwrap(); 
        Ok(())
    }
}
