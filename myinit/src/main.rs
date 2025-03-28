use myinit::{
    Error,
    subsystems::{
        fs,
        process::{DefaultProcessManager, ProcessManager},
        services::{ServiceSpec, supervisor::Supervisor},
    },
};

fn main() -> Result<(), Error> {
    fs::mount_essential()?;

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
