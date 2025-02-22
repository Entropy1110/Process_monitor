use nix::unistd::Pid;

pub fn handle_file_syscalls(pid: Pid, syscall: u64, regs: &libc::user_regs_struct) {
    match syscall {
        2 => println!("PID {} called open({:#x})", pid, regs.rdi),  // open() 시스템 콜 감지
        0 => println!("PID {} called read()", pid),  // read() 시스템 콜 감지
        1 => println!("PID {} called write()", pid), // write() 시스템 콜 감지
        _ => {}
    }
}