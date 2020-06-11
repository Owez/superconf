# superconf

A barebones configuration file made for low-dependancy rust applications.

## Usage

Add to your `Cargo.toml` file:

```toml
[dependancies]
superconf = "0.1.0"
```

Then you can parse a basic string like so:

```rust
use superconf::parse_str;

let input = "my_key my_value";

println!("Outputted HashMap: {:#?}", parse_str(input).unwrap());
```

## Example

Here is a complete syntax demonstration:

```none
# comments are like this
# no spaces are allowed in keys or values
# comments can only be at the start of lines, no end of line comments here

# my_key is the key, my_value is the value
my_key the_value

# you can use spaces, just have to be backslashed
your_path /home/user/Cool\ Path/x.txt
```

You can find more examples in the `examples/` directory next to this readme.

## Config Conventions

Some conventions commonly used for superconf files:

- The file naming scheme is `snake_case`
- All superconf files should end in the `.sc` file extension
- Try to document each line with a comment
- If commented, space each config part with an empty line seperating it from others. If it is undocumented, you may bunch all config parts together 

## Motives

Made this as a quick custom parser to challange myself a bit and to use for a quick-n-dirty configuration format in the future. It's not the best file format in the world but it gets the job done.
