use myinit::Error;
use myinit::subsystems::fs;
use myinit::subsystems::process::{DefaultProcessManager, ProcessManager};
use myinit::subsystems::services::ServiceSpec;
use myinit::subsystems::services::supervisor::Supervisor;

fn main() -> Result<(), Error> {
    fs::mount_essential()?;
    fs::setup_env();

    let pm = DefaultProcessManager;
    let mut sv = Supervisor::new(pm);

    sv.start_service(ServiceSpec {
        name: "debug-shell".into(),
        command: "/bin/sh".into(),
        deps: Vec::new(),
    })?;

    loop {
        sv.get_process_manager_mut().reap()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
