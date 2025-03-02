use libc::ESRCH;
use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::sys::ptrace::Options;
use nix::unistd::Pid;
use std::collections::HashMap;
use crate::file_monitor::handle_file_syscalls;
use crate::network_monitor::handle_network_syscalls;
use crate::security_monitor::handle_security_syscalls;

pub fn trace_process(pid: Pid, verbose: u8) {
    println!("Attaching to PID {}...", pid);
    ptrace::attach(pid).expect("Failed to attach to process");
    waitpid(pid, None).unwrap();

    ptrace::setoptions(
        pid,
        Options::PTRACE_O_TRACEEXEC | Options::PTRACE_O_TRACEFORK | Options::PTRACE_O_TRACEVFORK,
    ).expect("Failed to set ptrace options");

    let mut active_syscalls: HashMap<Pid, u64> = HashMap::new();

    loop {
        ptrace::syscall(pid, None).unwrap();
        if waitpid(pid, None).is_err() {
            eprintln!("Process {} exited.", pid);
            break;
        }

        let regs_before = match ptrace::getregs(pid) {
            Ok(regs) => regs,
            Err(err) => {
                if err as i32 == ESRCH {
                    eprintln!("Finished getting registers after syscall for PID {}: {:?}", pid, err);
                }
                else { eprintln!("Failed to get registers before syscall for PID {}: {:?}", pid, err); }
                break;
            }
        };

        let syscall = regs_before.orig_rax;
        active_syscalls.insert(pid, syscall);

        let mut detected = false;

        // 특정 syscall 감지
        detected |= handle_file_syscalls(pid, syscall, &regs_before);
        detected |= handle_network_syscalls(pid, syscall, &regs_before);
        detected |= handle_security_syscalls(pid, syscall, &regs_before);

        if verbose > 0 || detected {
            println!(
                "↪️ PID {}: Entering syscall {} (rdi: {:#x}, rsi: {:#x}, rdx: {:#x})",
                pid, syscall, regs_before.rdi, regs_before.rsi, regs_before.rdx
            );
        }

        ptrace::syscall(pid, None).unwrap();
        if waitpid(pid, None).is_err() {
            eprintln!("Process {} exited.", pid);
            break;
        }

        let regs_after = match ptrace::getregs(pid) {
            Ok(regs) => regs,
            Err(err) => {
                eprintln!("Finished getting registers after syscall for PID {}: {:?}", pid, err);
                break;
            }
        };

        if let Some(&syscall) = active_syscalls.get(&pid) {
            if verbose > 0 || detected {
                println!(
                    "⬅️ PID {}: Exiting syscall {} -> return value: {:#x}",
                    pid, syscall, regs_after.rax
                );
            }
            active_syscalls.remove(&pid);
        }
    }
}
