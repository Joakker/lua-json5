use mlua::{Error::ExternalError, IntoLua, Lua, Result, Value as LuaValue};
use pest::Parser;
use pest::iterators::Pair;
use std::collections::HashMap;
use std::sync::Arc;

use crate::val::Value;

#[derive(pest_derive::Parser)]
#[grammar = "json5.pest"]
struct Json5Parser;

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

fn parse_number(s: &str) -> f64 {
    let (negative, unsigned) = match s.as_bytes()[0] {
        b'-' => (true, &s[1..]),
        b'+' => (false, &s[1..]),
        _ => (false, s),
    };
    let value = match unsigned.get(..2) {
        // `f64::from_str` does not read hexadecimal, so 0x1F has to be read by hand.
        Some("0x" | "0X") => {
            let digits = &unsigned[2..];
            match u128::from_str_radix(digits, 16) {
                Ok(n) => n as f64,
                // more than 32 hex digits
                Err(_) => digits
                    .chars()
                    .fold(0.0, |acc, c| acc * 16.0 + c.to_digit(16).unwrap() as f64),
            }
        }
        _ => unsigned.parse().unwrap(),
    };
    if negative { -value } else { value }
}

#[test]
fn test_hex_number() {
    let cases = [
        ("0x1F", 31.0),
        ("0XfF", 255.0),
        ("+0x10", 16.0),
        ("-0xA", -10.0),
        ("0x0", 0.0),
        ("0xFFFFFFFFFFFFFFFF", 18446744073709551615.0),
    ];
    for (input, expected) in cases {
        let mut pairs = Json5Parser::parse(Rule::text, input).unwrap();
        match parse_pair(pairs.next().unwrap()) {
            Value::Number(n) => assert_eq!(n, expected, "{input}"),
            _ => panic!("{input} is not a number"),
        }
    }
}

#[test]
fn test_decimal_number() {
    let cases = [
        ("1", 1.0),
        ("+1.5", 1.5),
        ("-.5", -0.5),
        ("5.", 5.0),
        ("1e3", 1000.0),
        ("-Infinity", f64::NEG_INFINITY),
    ];
    for (input, expected) in cases {
        let mut pairs = Json5Parser::parse(Rule::text, input).unwrap();
        match parse_pair(pairs.next().unwrap()) {
            Value::Number(n) => assert_eq!(n, expected, "{input}"),
            _ => panic!("{input} is not a number"),
        }
    }
    let mut pairs = Json5Parser::parse(Rule::text, "-NaN").unwrap();
    match parse_pair(pairs.next().unwrap()) {
        Value::Number(n) => assert!(n.is_nan()),
        _ => panic!("-NaN is not a number"),
    }
}

fn parse_pair(pair: Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::array => Value::Array(pair.into_inner().map(parse_pair).collect()),
        Rule::null => Value::Null,
        Rule::string => Value::String(parse_str(pair)),
        Rule::number => Value::Number(parse_number(pair.as_str())),
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
            let mut m = HashMap::with_capacity(pairs.len());
            for (k, v) in pairs {
                m.insert(k, v);
            }
            Value::Object(m)
        }
        _ => unreachable!(),
    }
}

pub fn parse(lua: &Lua, data: String) -> Result<LuaValue> {
    let data = match Json5Parser::parse(Rule::text, data.as_str()) {
        Ok(mut data) => data.next().unwrap(),
        Err(err) => return Err(ExternalError(Arc::new(err))),
    };
    parse_pair(data).into_lua(lua)
}
