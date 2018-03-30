use rlua::{UserData, UserDataMethods, Lua, Result};
use convention::Convention;

struct Api;

impl Api {
    pub fn require(&self, _lua: &Lua, module: String) {
        info!("Loading lua module: {}", module)
    }
}

pub fn install(lua: &Lua, convention: &Convention) -> Result<()> {
    let globals = lua.globals();
    globals.set("Yahmc", Api)?;
    return Ok(())
}

impl UserData for Api {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method("require", |lua, api, (module): (String)| {
            api.require(lua, module);
            Ok("<todo: actually load module>")
        });
    }
}
