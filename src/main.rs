mod tracer;
mod file_monitor;
mod network_monitor;
mod security_monitor;

use structopt::StructOpt;
use nix::unistd::Pid;
use std::process::Command;

/// CLI 옵션 구조체 정의
#[derive(StructOpt, Debug)]
#[structopt(name = "syscall_monitor")]
struct Opt {
    /// target PIDs
    #[structopt(short, long)]
    pid: Option<i32>,

    /// Program path to run
    #[structopt(name = "PROGRAM", required_unless = "pid")]
    program: Option<String>,

    /// Arguments for the program
    #[structopt(name = "ARGS", last = true)]
    args: Vec<String>,

    /// if set, prints all of the syscalls the program called
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let opt = Opt::from_args();
    
    let pid = if let Some(pid) = opt.pid {
        println!("Monitoring existing PID: {}", pid);
        Pid::from_raw(pid)
    } else if let Some(program) = opt.program {
        println!("Starting and monitoring program: {}", program);
        let child = Command::new(program)
            .args(&opt.args)
            .spawn()
            .expect("Failed to start process");
        Pid::from_raw(child.id() as i32)
    } else {
        eprintln!("Error: Either a PID or a program to execute must be provided.");
        return;
    };

    tracer::trace_process(pid, opt.verbose);
}
