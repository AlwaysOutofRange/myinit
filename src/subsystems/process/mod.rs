pub mod reaper;
pub mod spawn;

pub trait ProcessManager {
    fn reap(&mut self) -> Result<(), crate::Error>;
    fn start(&self, cmd: &str) -> Result<u32, crate::Error>;
}

pub struct DefaultProcessManager;

impl ProcessManager for DefaultProcessManager {
    fn reap(&mut self) -> Result<(), crate::Error> {
        reaper::reap()
    }

    fn start(&self, cmd: &str) -> Result<u32, crate::Error> {
        spawn::spawn_process(cmd)
    }
}
