#![no_std]

extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub enum SuperError {
    /// When an item being parsed by [SuperItem] is empty, this is ignored by
    /// [Parse] implementation for the [SuperValue] parsing
    EmptyItem,
}

pub trait Parse<'a>: Sized {
    fn parse(input: &'a str) -> Result<Self, SuperError>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum SuperValue<'a> {
    Nothing,
    Name(&'a str),
    Bool(bool),
    Integer(i64),
    List(Vec<SuperValue<'a>>),
    Group(Vec<SuperItem<'a>>),
}

impl<'a> Parse<'a> for SuperValue<'a> {
    fn parse(input: &'a str) -> Result<Self, SuperError> {
        match input.trim() {
            "true" => Ok(Self::Bool(true)),
            "false" => Ok(Self::Bool(false)),
            trimmed => match trimmed.len() {
                0 => Ok(Self::Nothing),
                1 => Ok(num_or_name(trimmed)),
                _ => {
                    let mut trimmed_chars = trimmed.chars();
                    match (trimmed_chars.next().unwrap(), trimmed_chars.last().unwrap()) {
                        ('[', ']') => todo!("list"),
                        ('{', '}') => todo!("group"),
                        _ => Ok(num_or_name(trimmed)),
                    }
                }
            },
        }
    }
}

fn num_or_name<'a>(input: &'a str) -> SuperValue<'a> {
    match input.parse() {
        Ok(found) => SuperValue::Integer(found),
        Err(_) => SuperValue::Name(input),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuperItem<'a> {
    pub key: &'a str,
    pub value: SuperValue<'a>,
}

impl<'a> Parse<'a> for SuperItem<'a> {
    fn parse(input: &'a str) -> Result<Self, SuperError> {
        let (key, value) = flipflop_once(input, ' ').ok_or(SuperError::EmptyItem)?;

        Ok(Self {
            key,
            value: SuperValue::parse(value)?,
        })
    }
}

/// Flipflops a boolean to ensure that the `sep` value cannot be used if a
/// backspace is present properly
fn flipflop_once(input: &str, sep: char) -> Option<(&str, &str)> {
    // TODO: remove backslashes
    let mut flipflop = false;
    input.split_once(|c| {
        if c == '\\' {
            flipflop = true;
            false
        } else if c == sep {
            if flipflop {
                flipflop = false;
                false
            } else {
                true
            }
        } else {
            if flipflop {
                flipflop = false;
            }

            false
        }
    })
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuperConf<'a> {
    pub items: Vec<SuperItem<'a>>,
}

impl<'a> Parse<'a> for SuperConf<'a> {
    fn parse(input: &'a str) -> Result<Self, SuperError> {
        let mut items = Vec::new();

        for line in input.split('\n') {
            match SuperItem::parse(line) {
                Ok(item) => items.push(item),
                Err(SuperError::EmptyItem) => continue,
                Err(other) => return Err(other),
            }
        }

        Ok(Self { items })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse() {
        SuperConf::parse("loop 10\n\nhello there").unwrap();
        SuperConf::parse("loop 10\n\nhello there").unwrap();
        SuperConf::parse("loop 10\nloop {hello: there, other 2334, final [2,4,324,2]}").unwrap();
    }

    #[test]
    fn spaces_in_keys() {
        assert_eq!(
            SuperItem::parse("hello\\ there true").unwrap(),
            SuperItem {
                key: "hello there",
                value: SuperValue::Bool(true)
            }
        );
    }
}
