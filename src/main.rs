mod tracer;
mod file_monitor;
mod network_monitor;
mod security_monitor;
mod logger;

use nix::unistd::Pid;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program> [args...]", args[0]);
        return;
    }

    // 실행할 프로그램과 인자 설정
    let program = &args[1];
    let program_args = &args[2..];

    // 자식 프로세스 실행
    let child = Command::new(program)
        .args(program_args)
        .spawn()
        .expect("Failed to start process");

    let pid = Pid::from_raw(child.id() as i32);
    println!("Monitoring PID: {}", pid);

    // 시스템 콜 추적 시작
    tracer::trace_process(pid);
}