use super::*;
use std::path::PathBuf;
use failure::Error;

pub struct DiskResourceManager {
    root: PathBuf
}

impl ResourceManager for DiskResourceManager {
    type R = DiskResourceReader;
    type W = DiskResourceWriter;

    fn get_resource(&self, path: &Path) -> Result<Resource> {
        Err(errors::ResourceError::ResourceNotFound("".to_string()))
    }

    fn reader() -> Self::R {
        return DiskResourceReader;
    }

    fn writer() -> Self::W {
        return DiskResourceWriter;
    }
}


pub struct DiskResourceReader;
pub struct DiskResourceWriter;

impl ResourceReader for DiskResourceReader {}
impl ResourceWriter for DiskResourceWriter {}