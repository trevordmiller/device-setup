use super::paths;
use pulldown_cmark::{html, Options, Parser};

pub fn clean() {
    // Removes any previously generated output
    paths::remove_dir(&paths::public());
}

pub fn build() {
    // Generates a static HTML bundle from markdown notes
    paths::create_dir(&paths::public());
    let markdown_input = "EmptySome **example** testing from trevordmiller.com";
    let parser = Parser::new_ext(markdown_input, Options::empty());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let html_output_in_document = format!(
        "
        <!DOCTYPE html>
        <html lang=\"en-US\">
            <head>
                <meta charset=\"utf-8\">
                <meta name=\"author\" content=\"Trevor D. Miller\">
                <meta name=\"description\" content=\"Personal website.\">
                <title>trevordmiller.com</title>
            </head>
            <body>
                <header>
                    <p>Header</p>
                </header>
                <main>
                    <article>
                        {}
                    </article>
                </main>
                <footer>
                    <p>Footer</p>
                </footer>
              </body>
        </html>
    ",
        &html_output
    );
    paths::create_file(
        &paths::public().join("index.html"),
        &html_output_in_document,
    );
}

pub fn configure() {
    // Adds a CNAME file for the host (GitHub Pages) and registrar (Hover) to use my custom domain name (trevordmiller.com)
    paths::create_file(&paths::cname(), "trevordmiller.com");
}

pub fn deploy() {
    // Pushes output to my GitHub Pages repo (https://github.com/trevordmiller/trevordmiller.github.io)
}
