//! A barebones configuration file made for low-dependency rust applications.
//!
//! # Usage
//!
//! Add to your `Cargo.toml` file:
//!
//! ```toml
//! [dependancies]
//! superconf = "0.3"
//! ```
//!
//! # Examples
//!
//! Default seperator (space ` `) demonstration:
//!
//! ```rust
//! use superconf::parse_str;
//!
//! let input = "my_key my_value";
//!
//! println!("Outputted HashMap: {:#?}", parse_str(input).unwrap());
//! ```
//!
//! Or if you'd like to use a custom seperator like `:` or `=`:
//!
//! ```rust
//! use superconf::parse_custom_sep;
//!
//! let input_equal = "custom=seperator";
//! let input_colon = "second:string";
//!
//! println!("Equals seperator: {:#?}", parse_custom_sep(input_equal, '=').unwrap());
//! println!("Colon seperator: {:#?}", parse_custom_sep(input_colon, ':').unwrap());
//! ```
//!
//! Here is a complete syntax demonstration:
//!
//! ```none
//! # comments are like this
//! # no seperators are allowed in keys or values
//! # comments can only be at the start of lines, no end of line comments here
//!
//! # my_key is the key, my_value is the value
//! my_key the_value
//!
//! # you can use seperators as plaintext, just have to be backslashed
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
//! Made this as a quick custom parser to challenge myself a bit and to use for
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

    /// When adding elements and two are named with the same key, e.g:
    ///
    /// ```superconf
    /// my_value original
    /// my_value this_will_error
    /// ```
    ElementExists(SuperValue),

    /// An IO error stemming from [parse_file].
    IOError(std::io::Error),
}

/// The possible value of the config file.
///
/// Ususally a [SuperValue::Single] if 1 is provided or [SuperValue::List] if 2
/// or more are.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SuperValue {
    /// A single element provided, e.g. `my_key element`
    Single(String),

    /// Multiple elements provided, e.g. `my_key first_element second_element`
    List(Vec<String>),
}

/// The type of token for the mini lexer
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum TokenType {
    Character(char),
    Seperator,
    Backslash,
    Comment,
}

/// Lexes input into [Vec]<[Vec]<[TokenType]>> (top level for line, 2nd level
/// for each char in line).
fn lex_str(conf: &str, seperator: char) -> Vec<Vec<TokenType>> {
    let mut output: Vec<Vec<TokenType>> = vec![];

    for line in conf.lines() {
        let mut buffer: Vec<TokenType> = vec![];

        for line_char in line.chars() {
            let got_token = if line_char == seperator {
                TokenType::Seperator
            } else {
                match line_char {
                    '#' => TokenType::Comment,
                    '\\' => TokenType::Backslash,
                    t => TokenType::Character(t),
                }
            };

            buffer.push(got_token);
        }

        output.push(buffer);
    }

    output
}

/// Similar to [parse_str] but can enter a custom seperator other then the
/// default ` ` (space) character
pub fn parse_custom_sep(
    conf: &str,
    seperator: char,
) -> Result<HashMap<String, SuperValue>, SuperError> {
    let mut output: HashMap<String, SuperValue> = HashMap::new();
    let tokens = lex_str(conf, seperator);

    let mut _expect_new_level = false; // if only 1 key was found in line, expect a new level

    for token_line in tokens {
        // TODO: use `_expect_new_level` up here

        let mut buffer = vec![String::new()];

        let mut ignore_special = false; // a catcher for special chars prefixed with 1 `\`
        let mut is_comment = false; // used for skipping appends to output

        for token in token_line {
            match token {
                TokenType::Comment => {
                    // say its a comment then exit line
                    is_comment = true;
                    break;
                }
                TokenType::Backslash => ignore_special = !ignore_special, // switch ignore special
                TokenType::Seperator => {
                    // handle a seperator, ensuring that `\`'s are handled
                    if ignore_special {
                        // add seperator to buffer if it was backslashed
                        buffer.last_mut().unwrap().push(' ');

                        ignore_special = false;
                    } else if !buffer.last().unwrap().is_empty() {
                        // add new string to value buffer
                        buffer.push(String::new())
                    }
                }
                TokenType::Character(c) => buffer.last_mut().unwrap().push(c),
            }
        }

        if is_comment {
            continue;
        }

        let key = buffer.remove(0);

        let final_value = match buffer.len() {
            0 => {
                _expect_new_level = true;
                continue;
            } // looks to be a new level, expect it
            1 => SuperValue::Single(buffer[0].clone()),
            _ => SuperValue::List(buffer),
        };

        match output.insert(key, final_value) {
            Some(element) => return Err(SuperError::ElementExists(element)),
            None => (),
        };
    }

    Ok(output)
}

/// Parses given &[str] `conf` input.
pub fn parse_str(conf: &str) -> Result<HashMap<String, SuperValue>, SuperError> {
    parse_custom_sep(conf, ' ')
}

/// An alias to the more common [parse_str], allowing for easy usage with
/// [String]s.
pub fn parse_string(conf: String) -> Result<HashMap<String, SuperValue>, SuperError> {
    parse_custom_sep(&conf, ' ')
}

/// Opens a [PathBuf]-type file and parses contents.
pub fn parse_file(conf_path: PathBuf) -> Result<HashMap<String, SuperValue>, SuperError> {
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

    /// Tests valid keys that include backstroke seperators as a torture test
    #[test]
    fn backstroke_seperator_torture() {
        let input = "my\\ key this\\ is\\ the\\ value";
        let mut exp_output = HashMap::new();
        exp_output.insert(
            "my key".to_string(),
            SuperValue::Single("this is the value".to_string()),
        );

        assert_eq!(exp_output, parse_str(input).unwrap())
    }

    /// Tests a realistic-looking path
    #[test]
    fn realistic_path() {
        let input = "your_path /home/user/Cool\\ Path/x.txt";

        let mut exp_output = HashMap::new();
        exp_output.insert(
            "your_path".to_string(),
            SuperValue::Single("/home/user/Cool Path/x.txt".to_string()),
        );

        assert_eq!(exp_output, parse_str(input).unwrap());
    }

    /// Tests that eol comments like  the `# hi` in:
    ///
    /// ```superconf
    /// my_key my_value # hi
    /// ```
    ///
    /// Work properly.
    #[test]
    fn eol_comment() {
        let input = "my_key my_value # eol comment";

        parse_str(input).unwrap();
    }

    /// Tests that lists properly work
    #[test]
    fn item_list() {
        let input = "my_key first_val second_val";

        let mut exp_out = HashMap::new();
        exp_out.insert(
            "my_key".to_string(),
            SuperValue::List(vec!["first_val".to_string(), "second_val".to_string()]),
        );
        assert_eq!(exp_out, parse_str(input).unwrap());
    }

    /// Ensures custom seperators are properly parsed
    #[test]
    fn custom_seperator() {
        let input = "arrow>demonstration";

        let mut exp_out = HashMap::new();
        exp_out.insert(
            "arrow".to_string(),
            SuperValue::Single(String::from("demonstration")),
        );
        assert_eq!(exp_out, parse_custom_sep(input, '>').unwrap());
    }
}
