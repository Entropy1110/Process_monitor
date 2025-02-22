use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use crate::file_monitor::handle_file_syscalls;
use crate::network_monitor::handle_network_syscalls;
use crate::security_monitor::handle_security_syscalls;

pub fn trace_process(pid: Pid) {
    loop {
        waitpid(pid, None).unwrap();
        let regs = ptrace::getregs(pid).unwrap();
        let syscall = regs.orig_rax; // 시스템 콜 번호

        // 모듈별 처리
        handle_file_syscalls(pid, syscall, &regs);
        handle_network_syscalls(pid, syscall, &regs);
        handle_security_syscalls(pid, syscall, &regs);

        ptrace::syscall(pid, None).unwrap();
    }
}