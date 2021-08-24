use mlua::{Error::ExternalError, Lua, Result, ToLua, Value as LuaValue};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::sync::Arc;

use crate::val::Value;

#[derive(pest_derive::Parser)]
#[grammar = "json5.pest"]
struct Json5Parser;

fn parse_pair(pair: Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::array => Value::Array(pair.into_inner().map(parse_pair).collect()),
        Rule::null => Value::Null,
        Rule::string => {
            let mut s = String::new();
            for l in pair.into_inner() {
                match l.as_rule() {
                    Rule::char_literal => s.push_str(l.as_str()),
                    Rule::nul_escape_sequence => s.push_str("\u{0000}"),
                    _ => unreachable!(),
                }
            }
            Value::String(s)
        }
        Rule::number => Value::Number(pair.as_str().parse().unwrap()),
        Rule::boolean => Value::Boolean(pair.as_str().parse().unwrap()),
        Rule::object => {
            let pairs = pair.into_inner().map(|pair| {
                let mut inner_rule = pair.into_inner();
                let name = inner_rule.next().unwrap().as_str().to_string();
                let value = parse_pair(inner_rule.next().unwrap());
                (name, value)
            });
            let mut m = HashMap::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            Value::Object(m)
        }
        _ => unreachable!(),
    }
}

pub fn parse<'lua>(lua: &'lua Lua, data: String) -> Result<LuaValue<'lua>> {
    let data = match Json5Parser::parse(Rule::text, data.as_str()) {
        Ok(mut data) => data.next().unwrap(),
        Err(err) => {
            return Err(ExternalError(Arc::new(err)));
        }
    };
    Ok(parse_pair(data).to_lua(lua)?)
}
