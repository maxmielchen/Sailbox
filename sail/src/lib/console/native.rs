use std::io::{self, Write};
use termios::{self, Termios};
use std::os::unix::io::AsRawFd;

pub fn read_hidden(prompt: &str) -> io::Result<String> {
    let mut term = Termios::from_fd(io::stdin().as_raw_fd())?;
    let old = term.clone();
    term.c_lflag &= !termios::ECHO;
    termios::tcsetattr(io::stdin().as_raw_fd(), termios::TCSANOW, &term)?;
    let mut stdout = io::stdout();
    write!(stdout, "{}", prompt)?;
    stdout.flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    termios::tcsetattr(io::stdin().as_raw_fd(), termios::TCSANOW, &old)?;
    println!();
    Ok(input.trim().to_string())
}

pub fn read_input(prompt: &str) -> io::Result<String> {
    let mut term = Termios::from_fd(io::stdin().as_raw_fd())?;
    let old = term.clone();
    term.c_lflag |= termios::ECHO;
    termios::tcsetattr(io::stdin().as_raw_fd(), termios::TCSANOW, &term)?;
    let mut stdout = io::stdout();
    write!(stdout, "{}", prompt)?;
    stdout.flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    termios::tcsetattr(io::stdin().as_raw_fd(), termios::TCSANOW, &old)?;
    Ok(input.trim().to_string())
}

