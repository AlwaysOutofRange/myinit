use std::{
    process::Command,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::{
    Error,
    subsystems::{process::ProcessManager, services::supervisor::Supervisor},
};

pub static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
pub static REBOOT_REQUESTED: AtomicBool = AtomicBool::new(false);

pub fn request_shutdown(reboot: bool) {
    SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
    REBOOT_REQUESTED.store(reboot, Ordering::SeqCst);
}

pub fn is_shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::SeqCst)
}

pub fn perform_shutdown(
    supervisor: Arc<Mutex<Supervisor<impl ProcessManager>>>,
) -> Result<(), Error> {
    print!("Performing shutdown...");

    {
        let mut sv = supervisor.lock().unwrap();
        for service_name in sv.list_services().iter().rev() {
            println!("Stopping service: {}", service_name);
            if let Err(e) = sv.stop_service(service_name) {
                eprintln!("Failed to stop service {}: {}", service_name, e.message);
            }
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(2));

    // Kill remaining processes with SIGTERM
    println!("Sending SIGTERM to all remaining processes");
    if let Err(e) = Command::new("kill").args(&["-TERM", "-1"]).status() {
        eprintln!("Failed to send SIGTERM to all remaining processes: {}", e);
    }

    // Wait for processes to exit
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Kill remaining processes with SIGKILL if they haven't exited
    println!("Sending SIGKILL to all remaining processes");
    if let Err(e) = Command::new("kill").args(&["-KILL", "-1"]).status() {
        eprintln!("Failed to send SIGKILL to all remaining processes: {}", e);
    }

    println!("Unmounting filesystems");

    super::fs::umount_fs("/proc")?;
    super::fs::umount_fs("/sys")?;
    super::fs::umount_fs("/dev")?;

    println!("Syncing filesystems");
    nix::unistd::sync();

    if REBOOT_REQUESTED.load(Ordering::SeqCst) {
        println!("Rebooting system...");

        match nix::sys::reboot::reboot(nix::sys::reboot::RebootMode::RB_AUTOBOOT) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to reboot system: {}", e);
                // TODO: Implement a fallback mechanism to reboot the system
            }
        }
    } else {
        println!("Shutting down system...");

        match nix::sys::reboot::reboot(nix::sys::reboot::RebootMode::RB_POWER_OFF) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to power off system: {}", e);
                // TODO: Implement a fallback mechanism to power off the system
            }
        }
    }

    // Its unreachable but its a fallback if not rebooting or powering off
    std::process::exit(0);
}
