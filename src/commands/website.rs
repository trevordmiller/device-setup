pub fn clean() {
    // Removes any previously generated ./public/
}

pub fn build() {
    // Generates a static HTML bundle from ./src/notes/ markdown to ./public/
}

pub fn configure() {
    // Adds a CNAME file for the host (GitHub Pages) and registrar (Hover) to use a custom domain name (trevordmiller.com) in ./public/
}

pub fn deploy() {
    // Pushes ./public/ to my GitHub Pages repo (https://github.com/trevordmiller/trevordmiller.github.io)
}
