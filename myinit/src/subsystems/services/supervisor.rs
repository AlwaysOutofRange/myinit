use crate::{Error, subsystems::process::ProcessManager};

use super::{ServiceHandle, ServiceSpec};

pub struct Supervisor<P: ProcessManager> {
    pm: P,
    services: Vec<ServiceHandle>,
}

impl<P: ProcessManager> Supervisor<P> {
    pub fn new(pm: P) -> Self {
        Self {
            pm,
            services: Vec::new(),
        }
    }

    pub fn start_service(&mut self, spec: ServiceSpec) -> Result<(), Error> {
        let pid = self.pm.start(&spec.command)?;
        self.services.push(ServiceHandle { pid, spec });

        Ok(())
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.iter().map(|s| s.spec.name.clone()).collect()
    }

    pub fn start_service_by_name(&mut self, name: &str) -> Result<(), Error> {
        // TODO: Load service spec from config
        // Dummy service
        let spec = ServiceSpec {
            name: name.to_string(),
            command: format!("/bin/{}", name),
            deps: Vec::new(),
        };

        self.start_service(spec)
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), Error> {
        if let Some(pos) = self.services.iter().position(|s| s.spec.name == name) {
            let service = &self.services[pos];
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(service.pid as i32),
                nix::sys::signal::Signal::SIGTERM,
            )
            .map_err(|e| Error {
                message: format!("Failed to stop service: {}", e),
                fatal: false,
            })?;

            self.services.remove(pos);
            Ok(())
        } else {
            Err(Error {
                message: format!("Service '{}' not found", name),
                fatal: false,
            })
        }
    }

    pub fn restart_service(&mut self, name: &str) -> Result<(), Error> {
        let spec = if let Some(service) = self.services.iter().find(|s| s.spec.name == name) {
            service.spec.clone()
        } else {
            Err(Error {
                message: format!("Service '{}' not found", name),
                fatal: false,
            })?
        };

        self.stop_service(&spec.name)?;
        self.start_service(spec)
    }

    pub fn get_process_manager(&self) -> &P {
        &self.pm
    }

    pub fn get_process_manager_mut(&mut self) -> &mut P {
        &mut self.pm
    }
}
