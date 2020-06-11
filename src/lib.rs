//! A barebones configuration file made for low-dependency rust applications.
//!
//! # Usage
//!
//! Add to your `Cargo.toml` file:
//!
//! ```toml
//! [dependancies]
//! superconf = "0.1"
//! ```
//!
//! Then you can parse a basic string like so:
//!
//! ```rust
//! use superconf::parse_str;
//!
//! let input = "my_key my_value";
//!
//! println!("Outputted HashMap: {:#?}", parse_str(input).unwrap());
//! ```
//!
//! # Example
//!
//! Here is a complete syntax demonstration:
//!
//! ```none
//! # comments are like this
//! # no spaces are allowed in keys or values
//! # comments can only be at the start of lines, no end of line comments here
//!
//! # my_key is the key, my_value is the value
//! my_key the_value
//!
//! # you can use spaces, just have to be backslashed
//! your_path /home/user/Cool\ Path/x.txt
//!
//! # you can also have multiple levels
//! # will be:
//! # {"other_key": {"in_level": "see_it_is", "second_level": {"another": "level"}}}
//! other_key
//!     in_level see_it_is
//!     second_level
//!         another level
//! ```
//!
//! # Config Conventions
//!
//! Some conventions commonly used for superconf files:
//!
//! - The file naming scheme is `snake_case`
//! - All superconf files should end in the `.super` file extension
//! - Try to document each line with a comment
//! - If commented, space each config part with an empty line seperating it from
//! others. If it is undocumented, you may bunch all config parts together
//!
//! # Motives
//!
//! Made this as a quick custom parser to challange myself a bit and to use for
//! a quick-n-dirty configuration format in the future. It's not the best file
//! format in the world but it gets the job done.

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Primary error enum for superconf, storing the common errors faced.
#[derive(Debug)]
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

    /// An IO error stemming from [parse_file].
    IOError(std::io::Error),
}

/// The type of token for the mini lexer
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum TokenType {
    Character(char),
    Space,
    Backslash,
    Comment,
}

/// Lexes input into [Vec]<[Vec]<[TokenType]>> (top level for line, 2nd level
/// for each char in line).
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

        let mut is_comment = false; // if detected a comment, used to skip adding to output
        let mut in_key_buf = true; // if it should be in [key_buf] or [val_buf]
        let mut ignore_special = false; // a catcher for special chars prefixed with 1 `\`
        let mut precomment_whitespace = false; // for detecting a 3rd element (`z` in `x y z`)

        for token in token_line {
            match token {
                TokenType::Comment => {
                    is_comment = true;
                    break;
                }
                TokenType::Backslash => ignore_special = !ignore_special,
                TokenType::Character(c) => {
                    ignore_special = false;

                    if in_key_buf {
                        if precomment_whitespace {
                            // after 2nd element with space in-between
                            return Err(SuperError::TooManyElements);
                        }

                        key_buf.push(c)
                    } else {
                        val_buf.push(c)
                    }
                }
                TokenType::Space => {
                    if ignore_special {
                        if in_key_buf {
                            key_buf.push(' ')
                        } else {
                            val_buf.push(' ')
                        }

                        ignore_special = false;
                    } else {
                        if !in_key_buf {
                            precomment_whitespace = true;
                        }

                        in_key_buf = false;
                    }
                }
            }
        }

        if is_comment {
            continue;
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

/// An alias to the more common [parse_str], allowing for easy usage with
/// [String]s.
pub fn parse_string(conf: String) -> Result<HashMap<String, String>, SuperError> {
    parse_str(&conf)
}

/// Opens a [PathBuf]-type file and parses contents.
pub fn parse_file(conf_path: PathBuf) -> Result<HashMap<String, String>, SuperError> {
    let mut file = match File::open(conf_path) {
        Ok(f) => f,
        Err(e) => return Err(SuperError::IOError(e)),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => parse_string(contents),
        Err(e) => Err(SuperError::IOError(e)),
    }
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

    /// Tests that comments are working properly
    #[test]
    fn comment_test() {
        let input = "# This is a line comment, should not return any output!";

        assert_eq!(HashMap::new(), parse_str(input).unwrap());
    }

    /// Tests valid keys that include backstroke spaces as a torture test
    #[test]
    fn backstroke_space_torture() {
        let input = "my\\ key this\\ is\\ the\\ value";
        let mut exp_output = HashMap::new();
        exp_output.insert("my key".to_string(), "this is the value".to_string());

        assert_eq!(exp_output, parse_str(input).unwrap())
    }

    /// Tests a realistic-looking path
    #[test]
    fn realistic_path() {
        let input = "your_path /home/user/Cool\\ Path/x.txt";

        let mut exp_output = HashMap::new();
        exp_output.insert(
            "your_path".to_string(),
            "/home/user/Cool Path/x.txt".to_string(),
        );

        assert_eq!(exp_output, parse_str(input).unwrap());
    }

    /// Tests that eol comments like  the `# hi` in:
    ///
    /// ```superconf
    /// my_key my_value # hi
    /// ```
    ///
    /// Work properly
    #[test]
    fn eol_comment() {
        let input = "my_key my_value # eol comment";

        parse_str(input).unwrap();
    }
}
