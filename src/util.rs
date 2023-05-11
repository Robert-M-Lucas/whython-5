use std::io::{stdin, stdout, Read, Write};
use std::mem::size_of;

/// Gets a `u8` from `memory` at the pointer
pub fn get_u8(pointer: &usize, memory: &[u8]) -> u8 {
    u8::from_le_bytes((&memory[*pointer..(*pointer + 1)]).try_into().unwrap())
}

/// Gets a `usize` from `memory` at the pointer
pub fn get_usize(pointer: &usize, memory: &[u8]) -> usize {
    usize::from_le_bytes(
        (&memory[*pointer..(*pointer + size_of::<usize>())])
            .try_into()
            .unwrap(),
    )
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
/// use whython_4::col_println;
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
/// use whython_4::col_print;
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
    stdout.write(b"Press enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
