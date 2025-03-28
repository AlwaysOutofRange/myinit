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

    pub fn get_process_manager(&self) -> &P {
        &self.pm
    }

    pub fn get_process_manager_mut(&mut self) -> &mut P {
        &mut self.pm
    }
}
