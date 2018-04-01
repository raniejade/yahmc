mod internals;
mod lua;


use rlua::{UserData, UserDataMethods, Lua, Result, Table};
use convention::Convention;

struct Api;

impl<'lua> Api {
    pub fn require(&self, lua: &Lua, module: String) -> Result<String> {
        let wrapper = lua.create_table()?;
        info!("Loading lua module: {}", module);

        return Ok(String::from("as"));
    }

    pub fn create_component(&self, lua: &'lua Lua, name: String, mt: Table<'lua>) -> Result<Table<'lua>> {
        let globals = lua.globals();
        let component = lua.create_table()?;

        let new = lua.create_function(|lua, (this, o): (Table, Option<Table>)| {
            let object = o.unwrap_or(lua.create_table()?);
            object.set_metatable(Some(this));
            Ok(object)
        })?;

        component.set("__index", mt)?;
        component.set("new", new)?;
        globals.set(name.to_owned(), component)?;
        
        return Ok(globals.get::<_, Table>(name)?);
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

        methods.add_method("createComponent", |lua, api, (name, o): (String, Table)| {
            api.create_component(lua, name, o)
        });
    }
}   
