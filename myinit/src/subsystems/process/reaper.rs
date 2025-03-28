use crate::Error;
use nix::{
    sys::wait::{WaitPidFlag, WaitStatus, waitpid},
    unistd::Pid,
};

pub fn reap() -> Result<(), Error> {
    loop {
        match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("Reaped process {} (status: {})", pid, status);
            }
            Ok(WaitStatus::Signaled(pid, sig, _)) => {
                println!("Process {} killed by signal {:?}", pid, sig);
            }
            Ok(WaitStatus::StillAlive) => break,
            Err(nix::Error::ECHILD) => break,
            Err(e) => return Err(e.into()),
            _ => continue,
        }
    }
    Ok(())
}
