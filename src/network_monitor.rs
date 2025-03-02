use nix::unistd::Pid;
use libc::user_regs_struct;

pub fn handle_network_syscalls(pid: Pid, syscall: u64, regs: &user_regs_struct) -> bool {
    match syscall {
        42 => {
            let socket_fd = regs.rdi;
            let sockaddr_ptr = regs.rsi;
            let addrlen = regs.rdx;
            println!(
                "\x1b[38;5;74m🌐 [Network Monitor] PID {}: connect() called (socket fd: {}, sockaddr ptr: {:#x}, addrlen: {})\x1b[0m",
                pid, socket_fd, sockaddr_ptr, addrlen
            );
            true
        }
        49 => {
            let socket_fd = regs.rdi;
            let sockaddr_ptr = regs.rsi;
            let addrlen = regs.rdx;
            println!(
                "\x1b[38;5;74m🌐 [Network Monitor] PID {}: bind() called (socket fd: {}, sockaddr ptr: {:#x}, addrlen: {})\x1b[0m",
                pid, socket_fd, sockaddr_ptr, addrlen
            );
            true
        }
        50 => {
            let socket_fd = regs.rdi;
            let backlog = regs.rsi;
            println!(
                "\x1b[38;5;74m🌐 [Network Monitor] PID {}: listen() called (socket fd: {}, backlog: {})\x1b[0m",
                pid, socket_fd, backlog
            );
            true
        }
        _ => {
            false
        }
    }
}
