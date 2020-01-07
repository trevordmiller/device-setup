use super::progress;

pub fn next() {
    println!("Writing a note so that I solidify the most important things I've learned.");

    // New note in:
    // notes/{category}
    // Markdown file
    // Permanant id in format "{category}/{kebab-case-topic}" like "rust/asynchronous-functions"
    // Human title in format "{Topic} in {Category}" like "Asynchronous functions in Rust"

    progress::pause();
}
