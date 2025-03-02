use nix::unistd::Pid;
use libc::user_regs_struct;

pub fn handle_file_syscalls(pid: Pid, syscall: u64, regs: &user_regs_struct) -> bool{
    match syscall {
        2 => {
            let path_ptr = regs.rdi;
            let flags = regs.rsi;
            println!(
                "\x1b[38;5;78m📂 [File Monitor] PID {}: open() called (path ptr: {:#x}, flags: {:#x})\x1b[0m",
                pid, path_ptr, flags
            );
            true
        }
        0 => {
            let fd = regs.rdi;
            let buffer_ptr = regs.rsi;
            let size = regs.rdx;
            println!(
                "\x1b[38;5;78m📂 [File Monitor] PID {}: read() called (fd: {}, buffer: {:#x}, size: {})\x1b[0m",
                pid, fd, buffer_ptr, size
            );
            true
        }
        1 => {
            let fd = regs.rdi;
            let buffer_ptr = regs.rsi;
            let size = regs.rdx;
            println!(
                "\x1b[38;5;78m📂 [File Monitor] PID {}: write() called (fd: {}, buffer: {:#x}, size: {})\x1b[0m",
                pid, fd, buffer_ptr, size
            );
            true
        }
        _ => {
            false
        }
    }
}
