use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;

pub fn trace_process(pid: Pid) {
    // 프로세스에 ptrace 연결
    println!("Attaching to PID {}...", pid);
    ptrace::attach(pid).expect("Failed to attach to process");
    println!("Attached!");
    

    loop {
        match waitpid(pid, None) {
            Ok(_) => {
                let regs = ptrace::getregs(pid);
                match regs {
                    Ok(regs) => {
                        let syscall = regs.orig_rax;
                        println!("PID {} called syscall: {}", pid, syscall);
                    }
                    Err(err) => {
                        eprintln!("Failed to get registers for PID {}: {:?}", pid, err);
                        break;
                    }
                }
                ptrace::syscall(pid, None).unwrap();
            }
            Err(err) => {
                eprintln!("Failed to wait for PID {}: {:?}", pid, err);
                break;
            }
        }
    }
}