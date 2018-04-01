use std::path::PathBuf;
use convention::Convention;

use rlua::{Lua, Result};
use api;

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
    pub fn run(self) -> Result<()> {
        let convention = Convention::create(self.settings.root);
        info!("Starting engine ...");
        info!("{:#?}", convention);
        let lua = Lua::new();
        api::install(&lua, &convention)?;
        lua.eval::<()>(
            r#"
            local Vector = {x = 0, y = 0}
            local Component = Yahmc:createComponent("Vector", Vector)
            local v = Component:new()
            "#,
            None,
        )?;

        return Ok(())
    }
}