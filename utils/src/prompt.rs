//! This module implements prompting.

use libc::tcgetattr;
use libc::tcsetattr;
use libc::termios;
use libc::ECHO;
use libc::ECHOE;
use libc::ICANON;
use libc::STDIN_FILENO;
use libc::TCSANOW;
use libc::VMIN;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::mem::MaybeUninit;

// TODO Add line edition
/// Show a prompt. This function returns when a newline is received.
///
/// Arguments:
/// - `prompt` is the prompt's text. If `None`, the function uses the default text.
/// - `hidden` tells whether the input is hidden.
pub fn prompt(prompt: Option<&str>, hidden: bool) -> Option<String> {
    let prompt = prompt.unwrap_or("Password: ");

    // Save termios state
    let saved_termios = unsafe {
        let mut t: termios = MaybeUninit::zeroed().assume_init();
        tcgetattr(STDIN_FILENO, &mut t);
        t
    };

    if hidden {
        // Set temporary termios
        let mut termios = saved_termios;
        termios.c_lflag &= !(ICANON | ECHO | ECHOE);
        termios.c_cc[VMIN] = 1;

        unsafe {
            tcsetattr(STDIN_FILENO, TCSANOW, &termios);
        }
    }

    // Show prompt
    print!("{prompt}");
    let _ = io::stdout().flush();

    // Read input
    let input = io::stdin().lock().lines().next()?.unwrap_or(String::new());

    if hidden {
        println!();

        // Restore termios state
        unsafe {
            tcsetattr(STDIN_FILENO, TCSANOW, &saved_termios);
        }
    }

    Some(input)
}
