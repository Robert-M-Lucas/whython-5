pub mod must_use_option;
pub mod ref_or_box;

use std::io::{stdin, stdout, Read, Write};

#[cfg(target_pointer_width = "128")]
pub const USIZE_BYTES: usize = 16;
#[cfg(target_pointer_width = "64")]
pub const USIZE_BYTES: usize = 8;
#[cfg(target_pointer_width = "32")]
pub const USIZE_BYTES: usize = 4;
#[cfg(target_pointer_width = "16")]
pub const USIZE_BYTES: usize = 2;

/// Gets a `u8` from `memory` at the pointer
// pub fn get_u8(pointer: &usize, memory: &[u8]) -> u8 {
//     u8::from_le_bytes((&memory[*pointer..(*pointer + 1)]).try_into().unwrap())
// }

/// Gets a `usize` from `memory` at the pointer
pub fn get_usize(pointer: &mut usize, memory: &[u8]) -> usize {
    let u = usize::from_le_bytes(
        (&memory[*pointer..(*pointer + USIZE_BYTES)])
            .try_into()
            .unwrap(),
    );

    *pointer += USIZE_BYTES;

    u
}

// /// Evaluates to the `Ok` value or returns `Err(e)`
// #[macro_export] macro_rules! propagate_error {
//     ($result: expr) => {
//         match $result {
//             Err(e) => return Err(e),
//             Ok(value) => value
//         }
//     };
// }

/// Prints formatted text
///
/// # Arguments
/// * `colour` / `colours` - List of formatting for the text
/// * `format! args` - Remaining args formatted like `format!`
///
/// # Example
/// ```
/// use whython_5::col_println;
///
/// col_println!((red, bold), "Sample Text: [{}, {}]", "Text one", "text two");
/// ```
#[macro_export]
macro_rules! col_println {
    ($color: ident, $($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{}", format!($($arg)*).$color())
        }
    };
    (($($col_args:tt),*), $($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{}", format!($($arg)*)$(.$col_args())*)
        }
    };
}

/// Prints formatted text
///
/// # Arguments
/// * `colour` / `colours` - List of formatting for the text
/// * `format! args` - Remaining args formatted like `format!`
///
/// # Example
/// ```
/// use whython_5::col_print;
///
/// col_print!((red, bold), "Sample Text: [{}, {}]", "Text one", "text two");
/// ```
#[macro_export]
macro_rules! col_print {
    ($color: ident, $($arg:tt)*) => {
       {
           use colored::Colorize;
           print!("{}", format!($($arg)*).$color())
       }
    };
    (($($col_args:tt),*), $($arg:tt)*) => {
        {
            use colored::Colorize;
            print!("{}", format!($($arg)*)$(.$col_args())*)
        }
    };
}

#[macro_export]
macro_rules! bx {
    ($expr: expr) => {
        Box::new($expr)
    };
}

#[macro_export]
macro_rules! unwrap_println_err_return {
    ($expr: expr) => {
        match $expr {
            Ok(value) => value,
            Err(e) => {
                $crate::col_println!((red, bold), "{}", e);
                return;
            }
        }
    };
}

/// Prints a warning
pub fn warn(warning: &str) {
    col_println!((yellow, bold), "[WARNING]: {}", warning);
}

/// Prints information
pub fn info(info: &str) {
    col_println!((blue, bold), "[INFO]: {}", info);
}

/// Waits for enter key to be pressed
#[allow(clippy::unused_io_amount)]
pub fn pause() {
    let mut stdout = stdout();
    stdout
        .write(b"Press enter to exit...")
        .expect("Stdout write failed");
    stdout.flush().expect("Stdout flush failed");
    stdin().read(&mut [0]).expect("Stdin read failed");
}

pub fn substring(string: &str, start: usize, len: usize) -> String {
    string.chars().skip(start).take(len).collect()
}

pub fn join_reference_name(name: &[String]) -> String {
    let mut joined_name = String::new();
    for (i, string) in name.iter().enumerate() {
        joined_name += string;
        if i != name.len() - 1 {
            joined_name.push('.');
        }
    }
    joined_name
}

pub fn join_file_name(name: &[String]) -> String {
    let mut joined_name = String::new();
    for (i, string) in name.iter().enumerate() {
        joined_name += string;

        if i < name.len() - 2 {
            joined_name.push('/');
        } else if i == name.len() - 2 {
            joined_name.push('.')
        }
    }
    joined_name
}
