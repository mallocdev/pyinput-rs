use std::{fmt::Display, io::Write};

/// # Prompt the user for input and return the entered text.
///
/// # Example:
/// ```
/// use pyinput::input;
///
/// fn main() {
///     let input = input("Enter your name: ").unwrap();
///     println!("Hello, {}!", input);
/// }
/// ```
pub fn input(txt: impl Display) -> Result<String, std::io::Error> {
    print!("{}", txt);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    Ok(buffer.trim().to_string())
}
