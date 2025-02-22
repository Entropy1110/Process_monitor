use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_syscall(pid: i32, syscall_name: &str) {
    let log_entry = json!({
        "pid": pid,
        "syscall": syscall_name
    });

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("syscall_log.json")
        .unwrap();
    writeln!(file, "{}", log_entry).unwrap();
}