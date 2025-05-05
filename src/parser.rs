use mlua::{Error::ExternalError, IntoLua, Lua, Result, Value as LuaValue};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::sync::Arc;

use crate::val::Value;

#[derive(pest_derive::Parser)]
#[grammar = "json5.pest"]
struct Json5Parser;

// TODO(Joakker): Make this return a Result<String> instead of a naked String.
fn parse_str(pair: Pair<Rule>) -> String {
    let mut buf = Vec::<u16>::with_capacity(pair.as_str().len());
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::char_literal => buf.extend(p.as_str().encode_utf16()),
            Rule::nul_escape_sequence => buf.push(0),
            Rule::char_escape_sequence => match p.as_str() {
                "n" => buf.push(0xA),
                "r" => buf.push(0xD),
                "t" => buf.push(0x9),
                "b" => buf.push(0x8),
                "v" => buf.push(0xB),
                "f" => buf.push(0xC),
                k => buf.extend(k.encode_utf16()),
            },
            Rule::hex_escape_sequence => {
                let s = p.as_str();
                let hex = u8::from_str_radix(s, 16).unwrap_or(0);
                buf.push(hex as u16);
            }
            Rule::unicode_escape_sequence => {
                if let Ok(v) = u16::from_str_radix(p.as_str(), 16) {
                    buf.push(v)
                }
            }
            _ => unreachable!(),
        }
    }
    String::from_utf16_lossy(&buf)
}

#[test]
fn test_char_espace_sequence() {
    let mut pairs = Json5Parser::parse(Rule::string, r#""\t""#).unwrap();
    let s = parse_str(pairs.next().unwrap());
    assert_eq!(s, "\t")
}

#[test]
fn test_hex_espace_sequence() {
    let mut pairs = Json5Parser::parse(Rule::string, r#""\x0A""#).unwrap();
    let s = parse_str(pairs.next().unwrap());
    assert_eq!(s, "\n")
}

#[test]
fn test_unicode_espace_sequence_surrogate() {
    let mut pairs = Json5Parser::parse(Rule::string, r#""\ud834\udd1e""#).unwrap();
    let s = parse_str(pairs.next().unwrap());
    assert_eq!(s, "𝄞")
}

#[test]
fn test_unicode_espace_sequence() {
    let mut pairs = Json5Parser::parse(Rule::string, r#""\u000a""#).unwrap();
    let s = parse_str(pairs.next().unwrap());
    assert_eq!(s, "\n")
}

fn parse_pair(pair: Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::array => Value::Array(pair.into_inner().map(parse_pair).collect()),
        Rule::null => Value::Null,
        Rule::string => Value::String(parse_str(pair)),
        Rule::number => Value::Number(pair.as_str().parse().unwrap()),
        Rule::boolean => Value::Boolean(pair.as_str().parse().unwrap()),
        Rule::object => {
            let pairs = pair.into_inner().map(|pair| {
                let mut inner_rule = pair.into_inner();
                let name = {
                    let pair = inner_rule.next().unwrap();
                    match pair.as_rule() {
                        Rule::identifier => pair.as_str().to_string(),
                        Rule::string => parse_str(pair),
                        _ => unreachable!(),
                    }
                };
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
        Err(err) => return Err(ExternalError(Arc::new(err))),
    };
    Ok(parse_pair(data).into_lua(lua)?)
}
