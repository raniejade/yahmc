use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Convention {
    pub entities_dir: PathBuf,
    pub processors_dir: PathBuf,
    pub scripts_dir: PathBuf
}

impl Convention {
    pub fn create(root: &PathBuf) -> Convention {
        let entities_dir = root.join(Path::new("entities"));
        let processors_dir = root.join(Path::new("processors"));
        let scripts_dir = root.join(Path::new("scripts"));
        return Convention {
            entities_dir,
            processors_dir,
            scripts_dir
        };
    }
}