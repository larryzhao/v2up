use std::fs;

pub fn read_pidfile(path: &str) -> i32 {
    let pid_str = fs::read_to_string(path).unwrap_or(String::from("0"));
    let pid = pid_str.trim().parse().unwrap();
    pid
}

pub fn write_pidfile(path: &str, pid: &str) {
    fs::write(path, pid);
}
