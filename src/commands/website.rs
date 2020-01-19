use super::paths;

pub fn clean() {
    // Removes any previously generated output
    paths::remove_dir(&paths::public());
}

pub fn build() {
    // Generates a static HTML bundle from markdown notes
    paths::create_dir(&paths::public());
}

pub fn configure() {
    // Adds a CNAME file for the host (GitHub Pages) and registrar (Hover) to use my custom domain name (trevordmiller.com)
}

pub fn deploy() {
    // Pushes output to my GitHub Pages repo (https://github.com/trevordmiller/trevordmiller.github.io)
}
