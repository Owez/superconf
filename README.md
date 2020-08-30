# superconf

A barebones configuration file made for low-dependency rust applications.

## Usage

Add to your `Cargo.toml` file:

```toml
[dependancies]
superconf = "0.3"
```

## Examples

Default seperator (space ` `) demonstration:

```rust
use superconf::parse_str;

let input = "my_key my_value";

println!("Outputted HashMap: {:#?}", parse_str(input).unwrap());
```

Or if you'd like to use a custom seperator like `:` or `=`:

```rust
use superconf::parse_custom_sep;

let input_equal = "custom=seperator";
let input_colon = "second:string";

println!("Equals seperator: {:#?}", parse_custom_sep(input_equal, '=').unwrap());
println!("Colon seperator: {:#?}", parse_custom_sep(input_colon, ':').unwrap());
```

Here is a complete syntax demonstration:

```none
# comments are like this
# no seperators are allowed in keys or values
# comments can only be at the start of lines, no end of line comments here

# my_key is the key, my_value is the value
my_key the_value

# you can use seperators as plaintext, just have to be backslashed
your_path /home/user/Cool\ Path/x.txt

# you can also have multiple levels
# will be:
# {"other_key": {"in_level": "see_it_is", "second_level": {"another": "level"}}}
other_key
    in_level see_it_is
    second_level
        another level
```

## Config Conventions

Some conventions commonly used for superconf files:

- The file naming scheme is `snake_case`
- All superconf files should end in the `.super` file extension
- Try to document each line with a comment
- If commented, space each config part with an empty line seperating it from
others. If it is undocumented, you may bunch all config parts together

## Motives

Made this as a quick custom parser to challenge myself a bit and to use for
a quick-n-dirty configuration format in the future. It's not the best file
format in the world but it gets the job done.
