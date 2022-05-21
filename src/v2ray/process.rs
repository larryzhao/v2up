use std::fs;
use std::path::PathBuf;
use sysinfo::SystemExt;

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

    // pub fn start(this: &Self) -> Result(_, Error) {
    // }
    //
    // pub fn stop(this: &Self) -> Result(_, Error) {
    // }
}
