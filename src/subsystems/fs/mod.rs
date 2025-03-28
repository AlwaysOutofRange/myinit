use std::env;

use nix::{
    libc::setenv,
    mount::{MsFlags, mount},
};

use crate::Error;

pub fn mount_essential() -> Result<(), Error> {
    mount_fs("proc", "/proc", "proc", None)?;
    mount_fs("sysfs", "/sys", "sysfs", None)?;
    mount_fs("devtmpfs", "/dev", "devtmpfs", None)?;

    Ok(())
}

pub fn setup_env() {
    unsafe {
        setenv(
            "PATH".as_ptr() as *const i8,
            "/bin".as_ptr() as *const i8,
            true as i32,
        );
    }
    for (key, value) in env::vars() {
        println!("Env: {}={}", key, value);
    }

    println!("Contents of /bin:");
    if let Ok(entries) = std::fs::read_dir("/bin") {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("  {:?}", entry.path());
            }
        }
    } else {
        println!("  Could not read /bin directory!");
    }
}

fn mount_fs(source: &str, target: &str, fstype: &str, flags: Option<MsFlags>) -> Result<(), Error> {
    mount::<str, str, str, str>(
        Some(source),
        target,
        Some(fstype),
        flags.unwrap_or(MsFlags::empty()),
        None,
    )?;

    Ok(())
}
