use nix::mount::{MsFlags, mount, umount};

use crate::Error;

pub fn mount_essential() -> Result<(), Error> {
    mount_fs("proc", "/proc", "proc", None)?;
    mount_fs("sysfs", "/sys", "sysfs", None)?;
    mount_fs("devtmpfs", "/dev", "devtmpfs", None)?;

    Ok(())
}

fn mount_fs(source: &str, target: &str, fstype: &str, flags: Option<MsFlags>) -> Result<(), Error> {
    println!("Mounting {} on {}...", source, target);
    match mount::<str, str, str, str>(
        Some(source),
        target,
        Some(fstype),
        flags.unwrap_or(MsFlags::empty()),
        None,
    ) {
        Ok(_) => println!("Successfully mounted {} on {}", source, target),
        Err(e) => {
            return Err(Error {
                message: format!("Failed to mount {} on {}: {}", source, target, e),
                fatal: true,
            });
        }
    }

    Ok(())
}

pub(super) fn umount_fs(target: &str) -> Result<(), Error> {
    println!("Unmounting {}...", target);

    match umount(target) {
        Ok(_) => println!("Successfully unmounted {}", target),
        Err(e) => {
            return Err(Error {
                message: format!("Failed to unmount {}: {}", target, e),
                fatal: true,
            });
        }
    }

    Ok(())
}
