use rlua::{Lua, Result, Table};

static REGISTRY_NAME: &str = "__yamhc__registry";

pub struct ClassRegistry<'lua>(&'lua Lua);

impl<'lua> ClassRegistry<'lua> {
    pub fn create(lua: &'lua Lua) -> ClassRegistry<'lua> {
        let internal_registry = lua.create_table().unwrap();
        lua.globals().set(REGISTRY_NAME, internal_registry);
        return ClassRegistry(lua);
    }

    pub fn get(&self, name: &String) -> Option<Table> {
        let internal_registry = self.get_internal_registry();
        let result = internal_registry.get::<_, Table>(name.to_owned());
        return match result {
            Ok(table) => Some(table),
            Err(e) => None
        };
    }

    pub fn register(&self, name: &String, class: Table<'lua>) -> Result<()> {
        self.get_internal_registry()
            .set(name.to_owned(), class)?;
        return Ok(());
    }

    fn get_internal_registry(&self) -> Table {
        return self.0.globals().get::<_, Table>(REGISTRY_NAME).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register() {
        let lua = Lua::new();
        let registry = ClassRegistry::create(&lua);

        let name = String::from("MyClass");
        let class = lua.create_table().unwrap();

        registry.register(&name, class);

        assert!(registry.get(&name).is_some());
    }

    #[test]
    fn test_get_none() {
        let lua = Lua::new();
        let registry = ClassRegistry::create(&lua);

        let name = String::from("MyClass");

        assert!(registry.get(&name).is_none());
    }
}