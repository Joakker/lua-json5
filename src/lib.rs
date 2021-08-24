use mlua::{Lua, Result, Table};
pub mod parser;
pub mod val;

#[mlua::lua_module]
fn json5(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    exports.set("parse", lua.create_function(parser::parse)?)?;
    Ok(exports)
}
