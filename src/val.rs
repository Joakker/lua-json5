use mlua::{Lua, Nil, Result, ToLua, Value as LuaValue};
use std::collections::HashMap;

pub enum Value {
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    String(String),
    Number(f64),
    Boolean(bool),
}

impl<'lua> ToLua<'lua> for Value {
    fn to_lua(self, lua: &'lua Lua) -> Result<LuaValue<'lua>> {
        match self {
            Self::Null => Ok(Nil),
            Self::Array(a) => a.to_lua(lua),
            Self::String(s) => s.to_lua(lua),
            Self::Number(n) => n.to_lua(lua),
            Self::Boolean(b) => b.to_lua(lua),
            Self::Object(o) => o.to_lua(lua),
        }
    }
}
