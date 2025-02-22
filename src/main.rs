mod tracer;
mod file_monitor;
mod network_monitor;
mod security_monitor;
mod logger;

use nix::sys::ptrace;
use nix::unistd::{fork, ForkResult, Pid};
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program> [args...]", args[0]);
        return;
    }

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            ptrace::traceme().expect("Failed to ptrace");
            Command::new(&args[1]).args(&args[2..]).exec();
        }
        Ok(ForkResult::Parent { child }) => {
            println!("Monitoring PID: {}", child);
            tracer::trace_process(child);
        }
        Err(_) => eprintln!("Fork failed"),
    }
}