use std::path::PathBuf;
use convention::Convention;

pub struct RunnerSettings {
    root: PathBuf
}

impl RunnerSettings {
    pub fn create(root: PathBuf) -> RunnerSettings {
        return RunnerSettings {
            root: root
        };
    }
}

pub struct Runner {
    pub settings: RunnerSettings
}

impl Runner {
    pub fn run(&self) {
        let convention = Convention::create(&self.settings.root);
        info!("Running engine ...");
        info!("{:#?}", convention);
    }
}