use nix::unistd::Pid;

pub fn handle_network_syscalls(pid: Pid, syscall: u64, regs: &libc::user_regs_struct) {
    match syscall {
        42 => println!("PID {} called connect()", pid),  // connect() 감지
        49 => println!("PID {} called bind()", pid),     // bind() 감지
        50 => println!("PID {} called listen()", pid),   // listen() 감지
        _ => {}
    }
}