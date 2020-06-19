use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn heading(content: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    match writeln!(&mut stdout, "\n{}\n{}", content, underline(content, "=")) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    match stdout.reset() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

pub fn subheading(content: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    match writeln!(&mut stdout, "\n{}\n{}", content, underline(content, "-")) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    match stdout.reset() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

pub fn progress(content: String) {
    let mut stdout = io::stdout();

    match writeln!(&mut stdout, "\n{}", content) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

pub fn info(content: String) {
    let mut stdout = io::stdout();

    match writeln!(&mut stdout, "\n{}", content) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

fn underline(content: &str, symbol: &str) -> String {
    content.chars().map(|_| symbol).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underline() {
        assert_eq!(underline("Some line", "="), "=========");
        assert_eq!(underline("Another line", "-"), "------------");
    }
}
