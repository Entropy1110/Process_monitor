use nix::unistd::Pid;

pub fn handle_security_syscalls(pid: Pid, syscall: u64, regs: &libc::user_regs_struct) {
    match syscall {
        59 => println!("PID {} executed a process (execve)", pid),  // execve() 감지
        105 => println!("PID {} changed permissions (chmod)", pid), // chmod() 감지
        23 => println!("PID {} changed UID (setuid)", pid),         // setuid() 감지
        _ => {}
    }
}