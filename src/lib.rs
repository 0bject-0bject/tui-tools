// 
// Copyright 2023, [object Object]
// Licensed under MIT
//


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