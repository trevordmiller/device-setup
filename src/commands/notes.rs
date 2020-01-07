use std::io;

pub fn next() {
    println!("Writing a note so that I solidify the most important things I've learned.");

    // New note in:
    // notes/{category}
    // Markdown file
    // Permanant id in format "{category}/{kebab-case-topic}" like "rust/asynchronous-functions"
    // Human title in format "{Topic} in {Category}" like "Asynchronous functions in Rust"
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
