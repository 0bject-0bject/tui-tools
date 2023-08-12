// 
// Copyright 2023, [object Object]
// Licensed under MIT
//

use std::io::{self, Write};
/// Colorize text in a terminal, automatically detects if the terminal supports colors
/// 
/// example:
/// ```
/// use tui_tools::Colors;
/// 
/// println!("{}", "Hello World!".red());
/// 
/// ```
pub mod colors;
pub use colors::Colors;


// How do I get keyboard input without the user pressing the Enter key? - https://stackoverflow.com/a/73765863
extern {
    fn _getch() -> core::ffi::c_char;
}


/// Gets a single utf-8 character from the keyboard
/// 
/// example: 
/// ```
/// use tui_tools::getch;
/// 
/// let c = getch(); // 
/// 
/// println!("{}", c);
/// 
/// ```
pub fn getch() -> u8 {
    unsafe {
        _getch() as u8
    }
}

/// Gets a single utf-8 character from the keyboard as a char
pub fn getch_as_char() -> char {
    getch() as char
}

/// Prints a message and creates a input on the same line
pub fn same_line_input(msg: &str) -> String {
    print!("{}", msg);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub mod cls {
    /// Clears the screen and moves the cursor to the top left
    /// 
    /// example:
    /// ```
    /// use tui_tools::cls;
    /// 
    /// cls();
    /// 
    /// ```
    pub fn cls() {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    }
}

pub use cls::cls;