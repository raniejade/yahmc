use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Convention {
    pub root_dir: PathBuf,
    pub components_dir: PathBuf,
    pub processors_dir: PathBuf,
    pub scripts_dir: PathBuf,
}

impl Convention {
    pub fn create(root: PathBuf) -> Convention {
        let components_dir = root.join(Path::new("components"));
        let processors_dir = root.join(Path::new("processors"));
        let scripts_dir = root.join(Path::new("scripts"));
        return Convention {
            root_dir: root,
            components_dir,
            processors_dir,
            scripts_dir,
        };
    }
}
