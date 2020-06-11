//! A barebones configuration file made for low-dependancy rust applications.
//!
//! Under the hood, it uses a simple line-by-line parsing technique. Overall, superconf
//! is desgined to be fast to develop and "good enough" for a simple configuration
//! job requiring no more dependancies than the stdlib.
//!
//! If parsed successfully, this library will typically output a simple [HashMap]
//! as provided by [std::collections] with both `key` and `value` of said [HashMap]
//! being a [String].

use std::collections::HashMap;

/// Primary error enum for superconf, storing the common errors faced.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SuperError {
    /// When a line had a key but no value, e.g. `my_key`
    NoKey,

    /// When a line had a value but no key. This should ususally not happen when
    /// parsing due to the nature of the library.
    NoValue,

    /// When too many elements where given, e.g. `my_key my_value stickler`.
    ///
    /// **You may face this when accidently adding a space without a `\`
    /// beforehand!**
    TooManyElements,
}

/// The type of token for the mini lexer
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum TokenType {
    Character(char),
    Space,
    Backslash,
    Comment,
}

/// Lexes input into [Vec]<[Vec]<[TokenType]>> (top level for line, 2nd level for each char in line).
fn lex_str(conf: &str) -> Vec<Vec<TokenType>> {
    let mut output: Vec<Vec<TokenType>> = vec![];

    for line in conf.lines() {
        let mut buffer: Vec<TokenType> = vec![];

        for line_char in line.chars() {
            let got_token = match line_char {
                ' ' => TokenType::Space,
                '#' => TokenType::Comment,
                '\\' => TokenType::Backslash,
                t => TokenType::Character(t),
            };

            buffer.push(got_token);
        }

        output.push(buffer);
    }

    output
}

/// Parses given &[str] `conf` input.
pub fn parse_str(conf: &str) -> Result<HashMap<String, String>, SuperError> {
    let mut output: HashMap<String, String> = HashMap::new();
    let tokens = lex_str(conf);

    for token_line in tokens {
        let mut key_buf = String::new();
        let mut val_buf = String::new();

        let mut in_key_buf = true;
        let mut ignore_space = false;

        for token in token_line {
            match token {
                TokenType::Comment => break,
                TokenType::Backslash => ignore_space = !ignore_space,
                TokenType::Character(c) => {
                    if in_key_buf {
                        key_buf.push(c)
                    } else {
                        val_buf.push(c)
                    }
                }
                TokenType::Space => {
                    if !ignore_space {
                        if !in_key_buf {
                            return Err(SuperError::TooManyElements);
                        }

                        in_key_buf = false;
                    }
                }
            }
        }

        if key_buf.is_empty() {
            return Err(SuperError::NoKey);
        } else if val_buf.is_empty() {
            return Err(SuperError::NoValue);
        }

        output.insert(key_buf, val_buf);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic parsing capabilities of just 1 key-value
    #[test]
    fn basic_parse() {
        let input = "my_key my_value";

        parse_str(input).unwrap();
    }
}
