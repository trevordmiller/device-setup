use std::io;

pub fn next() {
    println!("Writing a note so that I solidify the most important things I've learned.");

    // New note, markdown file in notes/
    // Permanant id like "rust-asynchronous-functions"
    // Title like "Asynchronous functions in Rust"
    loop {
        println!("Enter 'Continue' to continue");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }

        if input.contains("Continue") {
            break;
        } else {
            continue;
        }
    }
}
