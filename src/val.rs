use mlua::{IntoLua, Lua, Nil, Result, Value as LuaValue};
use std::collections::HashMap;

pub enum Value {
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    String(String),
    Number(f64),
    Boolean(bool),
}

impl IntoLua for Value {
    fn into_lua(self, lua: &Lua) -> Result<LuaValue> {
        match self {
            Self::Null => Ok(Nil),
            Self::Array(a) => a.into_lua(lua),
            Self::String(s) => s.into_lua(lua),
            Self::Number(n) => n.into_lua(lua),
            Self::Boolean(b) => b.into_lua(lua),
            Self::Object(o) => o.into_lua(lua),
        }
    }
}
