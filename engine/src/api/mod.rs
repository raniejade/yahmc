use rlua::{UserData, UserDataMethods, Lua, Result, Table};
use convention::Convention;
use std::sync::Arc;

struct Api;

impl Api {
    pub fn require(&self, lua: &Lua, module: String) -> Result<()> {
        let wrapper = lua.create_table()?;
        info!("Loading lua module: {}", module);

        return Ok(());
    }

    pub fn create_component(&self, lua: &Lua, name: String, mt: Table) -> Result<()> {
        let globals = lua.globals();
        let component = lua.create_table()?;
        globals.set("__component", component)?;

        let new = lua.create_function(|lua, (o): (Option<Table>)| {
            let globals = lua.globals();
            let object = o.unwrap_or(lua.create_table()?);
            object.set("__index", globals.get::<_, Table>("__component")?)?;
            object.set_metatable(Some(globals.get::<_, Table>("__component")?));
            Ok(())
        });

        globals.set(name, globals.get::<_, Table>("__component")?)?;
        return Ok(());
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
            api.require(lua, module)
        });
    }
}   
