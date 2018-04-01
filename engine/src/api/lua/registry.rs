use rlua::{Lua, Result, Table};

static REGISTRY_NAME: &str = "__yamhc__registry";

pub struct ClassRegistry<'lua>(&'lua Lua);

impl<'lua> ClassRegistry<'lua> {
    pub fn create(lua: &'lua Lua) -> Result<ClassRegistry<'lua>> {
        let internal_registry = lua.create_table()?;
        lua.globals().set(REGISTRY_NAME, internal_registry)?;
        return Ok(ClassRegistry(lua));
    }

    pub fn get(&self, name: &String) -> Result<Table> {
        let internal_registry = self.get_internal_registry()?;
        return internal_registry.get::<_, Table>(name.to_owned())
    }

    pub fn register(&self, name: &String, class: Table<'lua>) -> Result<()> {
        self.get_internal_registry()?
            .set(name.to_owned(), class)?;
        return Ok(());
    }

    fn get_internal_registry(&self) -> Result<Table> {
        return Ok(self.0.globals().get::<_, Table>(REGISTRY_NAME)?);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register() {
        let lua = Lua::new();
        let registry = ClassRegistry::create(&lua).unwrap();

        let name = String::from("MyClass");
        let class = lua.create_table().unwrap();

        registry.register(&name, class);

        assert!(registry.get(&name).is_ok());
    }

    #[test]
    fn test_get_none() {
        let lua = Lua::new();
        let registry = ClassRegistry::create(&lua).unwrap();

        let name = String::from("MyClass");

        assert!(registry.get(&name).is_err());
    }
}