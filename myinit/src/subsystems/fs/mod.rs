use nix::mount::{MsFlags, mount};

use crate::Error;

pub fn mount_essential() -> Result<(), Error> {
    mount_fs("proc", "/proc", "proc", None)?;
    mount_fs("sysfs", "/sys", "sysfs", None)?;
    mount_fs("devtmpfs", "/dev", "devtmpfs", None)?;

    Ok(())
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
