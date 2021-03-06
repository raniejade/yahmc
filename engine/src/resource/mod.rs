mod disk;
mod errors;

pub use self::errors::Result;

use std::path::{Path, PathBuf};

pub struct Resource {
    pub path: PathBuf,
}

pub trait ResourceReader {}
pub trait ResourceWriter {}

pub trait ResourceManager {
    type R: ResourceReader;
    type W: ResourceWriter;

    fn get_resource(&self, path: &Path) -> Result<Resource>;

    fn reader() -> Self::R;
    fn writer() -> Self::W;
}
