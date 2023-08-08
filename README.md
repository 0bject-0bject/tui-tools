# tui-tools

## Description
Some tools I use for colorizing, and accepting input in a tui applications with minimal dependencies.

## Usage
Add the following to your Cargo.toml
```toml
[dependencies]
tui-tools = "0.1.0"
```

## Examples
Colorize a string, enables ansi on windows.
```rust
use tui_tools::Colors;

fn main() {
    println!("{}", "Hello World!".green());
}
```

Get input from the user.
```rust
use tui_tools::getch;

fn main() {
    let input = getch();
    println!("You pressed: {}", input as char);
}
```

Clear the screen.
```rust
use tui_tools::cls;

fn main() {
    cls();
}
```

## License
[MIT](https://choosealicense.com/licenses/mit/)

## Sources
- [How do I get keyboard input without the user pressing the Enter key?](https://stackoverflow.com/a/73765863)
