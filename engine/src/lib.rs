#![feature(pattern_parentheses)]

extern crate failure;
#[macro_use]
extern crate failure_derive;


#[macro_use]
extern crate log;
extern crate rlua;

#[cfg(test)]
extern crate float_cmp;

mod lua;
mod api;
mod resource;
mod convention;
pub mod runner;

use runner::{Runner, RunnerSettings};
use std::path::PathBuf;

pub struct Builder {
    root: PathBuf,
}

impl Builder {
    pub fn create(root: PathBuf) -> Builder {
        return Builder { root: root };
    }

    pub fn build(self) -> Runner {
        return Runner {
            settings: RunnerSettings::create(self.root),
        };
    }
}
