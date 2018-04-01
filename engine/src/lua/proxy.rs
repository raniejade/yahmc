use super::constants;
use rlua::{Lua, Result, Table};

pub struct ClassProxyBuilder<'lua>(&'lua Lua, Table<'lua>);

impl<'lua> ClassProxyBuilder<'lua> {
    pub fn create(lua: &'lua Lua, table: Table<'lua>) -> ClassProxyBuilder<'lua> {
        return ClassProxyBuilder(lua, table);
    }

    pub fn build(self, mt: Table) -> Result<Table<'lua>> {
        self.1.set(constants::metamethod::INDEX, mt);
        self.1
            .set_metatable(Some(self.1.get::<_, Table>(constants::metamethod::INDEX)?));

        let new = self.0
            .create_function(|lua, (this, o): (Table, Option<Table>)| {
                let object = o.unwrap_or(lua.create_table()?);
                object.set_metatable(Some(this));
                Ok(object)
            })
            .unwrap();

        self.1.set("new", new)?;
        return Ok(self.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::ApproxEq;
    use std::f32;

    #[test]
    fn test_constructor() {
        let lua = Lua::new();
        create_vector_class(&lua);

        let result = lua.exec::<Table>(
            r#"
            local v = Vector:new()
            v.x = 1
            v.y = 2
            return v
            "#,
            None,
        ).unwrap();

        assert_eq!(1, result.get::<_, i32>("x").unwrap());
        assert_eq!(2, result.get::<_, i32>("y").unwrap());
    }

    #[test]
    fn test_base_not_changed() {
        let lua = Lua::new();
        create_vector_class(&lua);

        let result = lua.exec::<Table>(
            r#"
            local v = Vector:new()
            v.x = 1
            v.y = 2
            return Vector:new()
            "#,
            None,
        ).unwrap();

        assert_eq!(0, result.get::<_, i32>("x").unwrap());
        assert_eq!(0, result.get::<_, i32>("y").unwrap());
    }

    #[test]
    fn test_method_call() {
        let lua = Lua::new();
        create_vector_class(&lua);

        let result = lua.exec::<f32>(
            r#"
            local v = Vector:new()
            v.x = 1
            v.y = 2
            return v:magnitude()
            "#,
            None,
        ).unwrap();

        let expected = (1.0f32 * 1.0f32 + 2.0f32 * 2.0f32).sqrt();

        assert!(expected.approx_eq(&result, 2.0 * f32::EPSILON, 2));
    }

    #[test]
    fn test_reload() {
        let lua = Lua::new();
        let vector_class = create_vector_class(&lua);

        let mt = lua.exec::<Table>(
            r#"
            local mt = {x = 0, y = 0}
            
            function mt:magnitude()
                return math.sqrt(self.x * self.x + self.y * self.y)
            end

            function mt:some_function()
                return self.x + self.y;
            end

            return mt
            "#,
            None,
        ).unwrap();

        // reload
        ClassProxyBuilder::create(&lua, vector_class)
            .build(mt)
            .unwrap();

        let result = lua.exec::<i32>(
            r#"
            local v = Vector:new()
            v.x = 1
            v.y = 2
            return v:some_function()
            "#,
            None,
        ).unwrap();

        assert_eq!(3, result);
    }

    #[test]
    fn test_reload_existing_instance() {
        let lua = Lua::new();
        let vector_class = create_vector_class(&lua);
        lua.eval::<()>(
            r#"
            v = Vector:new()
            v.x = 1
            v.y = 2
            "#,
            None,
        );

        let mt = lua.exec::<Table>(
            r#"
            local mt = {x = 0, y = 0}
            
            function mt:magnitude()
                return math.sqrt(self.x * self.x + self.y * self.y)
            end

            function mt:some_function()
                return self.x + self.y;
            end

            return mt
            "#,
            None,
        ).unwrap();

        // reload
        ClassProxyBuilder::create(&lua, vector_class)
            .build(mt)
            .unwrap();

        let result = lua.exec::<i32>(
            r#"
            return v:some_function()
            "#,
            None,
        ).unwrap();

        assert_eq!(3, result);
    }

    fn create_vector_class<'lua>(lua: &'lua Lua) -> Table<'lua> {
        return create_class(
            &lua,
            "Vector",
            r#"
            local mt = {x = 0, y = 0}
            
            function mt:magnitude()
                return math.sqrt(self.x * self.x + self.y * self.y)
            end

            return mt
            "#,
        );
    }

    fn create_class<'lua>(lua: &'lua Lua, name: &str, body: &str) -> Table<'lua> {
        let globals = lua.globals();
        let mt = lua.exec::<Table>(body, None).unwrap();

        let wrapper = lua.create_table().unwrap();
        let class = ClassProxyBuilder::create(&lua, wrapper).build(mt).unwrap();

        globals.set(name.to_owned(), class);

        return globals.get::<_, Table>(name.to_owned()).unwrap();
    }
}
