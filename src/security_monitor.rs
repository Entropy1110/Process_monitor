use nix::unistd::Pid;
use libc::user_regs_struct;

pub fn handle_security_syscalls(pid: Pid, syscall: u64, regs: &user_regs_struct) -> bool {
    match syscall {
        59 => {
            let path_ptr = regs.rdi;
            let argv_ptr = regs.rsi;
            let envp_ptr = regs.rdx;
            println!(
                "\x1b[38;5;9m⚠️  [Security Monitor] PID {}: execve() called (path ptr: {:#x}, argv ptr: {:#x}, envp ptr: {:#x})\x1b[0m",
                pid, path_ptr, argv_ptr, envp_ptr
            );
            true
        }
        105 => {
            let path_ptr = regs.rdi;
            let mode = regs.rsi;
            println!(
                "\x1b[38;5;9m⚠️  [Security Monitor] PID {}: chmod() called (path ptr: {:#x}, mode: {:#o})\x1b[0m",
                pid, path_ptr, mode
            );
            true
        }
        23 => {
            let uid = regs.rdi;
            println!(
                "\x1b[38;5;9m⚠️  [Security Monitor] PID {}: setuid() called (uid: {})\x1b[0m",
                pid, uid
            );
            true
        }
        _ => {
            false
        }
    }
}
