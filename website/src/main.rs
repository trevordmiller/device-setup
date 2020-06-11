use copy_dir::copy_dir;
use pulldown_cmark::{html, Options, Parser};
use std::fs;

mod paths;
mod printing;

fn main() {
    printing::heading("Building website.");
    clean();
    generate();
    configure();
}

fn clean() {
    printing::subheading("Cleaning build directory.");
    paths::remove_dir(&paths::build());
}

fn generate() {
    printing::subheading("Generating build directory.");
    paths::create_dir(&paths::build());
    generate_public();
    generate_home();
    generate_code();
    generate_articles();
    generate_videos();
    generate_resume();
}

fn generate_public() {
    match copy_dir(&paths::public(), &paths::build().join("public")) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}

fn generate_home() {
    let markdown_file_contents = match fs::read_to_string(&paths::content().join("index.md")) {
        Ok(contents) => contents,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    paths::create_file(
        &paths::build().join("index.html"),
        &markdown_to_html(&markdown_file_contents),
    );
}

fn generate_code() {
    let markdown_file_contents = match fs::read_to_string(&paths::content().join("code.md")) {
        Ok(contents) => contents,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    paths::create_dir(&paths::build().join("code"));
    paths::create_file(
        &paths::build().join("code").join("index.html"),
        &markdown_to_html(&markdown_file_contents),
    );
}

fn generate_articles() {
    paths::create_dir(&paths::build().join("articles"));
    match fs::read_dir(&paths::content().join("articles")) {
        Ok(markdown_files) => {
            let mut markdown_links_to_routes = Vec::new();

            for markdown_file in markdown_files {
                let markdown_file = match &markdown_file {
                    Ok(markdown_file) => markdown_file.path(),
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let markdown_file_contents = match fs::read_to_string(&markdown_file) {
                    Ok(contents) => contents,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let route = &paths::file_stem(markdown_file);
                paths::create_dir(&paths::build().join("articles").join(route));
                paths::create_file(
                    &paths::build()
                        .join("articles")
                        .join(route)
                        .join("index.html"),
                    &markdown_to_html(&markdown_file_contents),
                );

                let title = match markdown_file_contents.lines().next() {
                    Some(title) => &title[2..],
                    None => panic!("Cannot find a title in {}.", &route),
                };
                markdown_links_to_routes
                    .push(format!("- [{}.](/articles/{})", title, route).to_string());
            }

            let markdown_articles_index = format!(
                "# Articles\nMy posts on software development.\n{}",
                markdown_links_to_routes.join("\n")
            );

            paths::create_file(
                &paths::build().join("articles").join("index.html"),
                &markdown_to_html(&markdown_articles_index),
            );
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}

fn generate_videos() {
    let markdown_file_contents = match fs::read_to_string(&paths::content().join("videos.md")) {
        Ok(contents) => contents,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    paths::create_dir(&paths::build().join("videos"));
    paths::create_file(
        &paths::build().join("videos").join("index.html"),
        &markdown_to_html(&markdown_file_contents),
    );
}

fn generate_resume() {
    let markdown_file_contents = match fs::read_to_string(&paths::content().join("resume.md")) {
        Ok(contents) => contents,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    paths::create_dir(&paths::build().join("resume"));
    paths::create_file(
        &paths::build().join("resume").join("index.html"),
        &markdown_to_html(&markdown_file_contents),
    );
}

fn configure() {
    printing::subheading("Configuring build directory.");

    // Adds a CNAME file for the host (GitHub Pages) and registrar (Hover) to use my custom domain name (trevordmiller.com)
    paths::create_file(&paths::build().join("CNAME"), "trevordmiller.com");
}

fn markdown_to_html(markdown: &str) -> std::string::String {
    let parser = Parser::new_ext(markdown, Options::empty());
    let mut html = String::new();
    html::push_html(&mut html, parser);

    format!(
        "
        <!DOCTYPE html>
        <html lang='en-US'>
            <head>
                <title>trevordmiller.com</title>
                <meta name='author' content='Trevor D. Miller'>
                <meta name='description' content='Personal website.'>
                <meta charset='utf-8'>
                <meta name='viewport' content='width=device-width, initial-scale=1'>
                <link rel='stylesheet' href='/public/theme.css' />
            </head>
            <body>
                <header>
                    <nav>
                        <a href='/'>trevordmiller</a>
                        <a href='/articles/'>Articles</a>
                        <a href='/videos/'>Videos</a>
                        <a href='/code/'>Code</a>
                        <a href='/resume/'>Resume</a>
                    </nav>
                </header>
                <main>
                    {}
                </main>
                <footer>
                    <h2>Get my latest content by email</h2>
                    <p>I won't sell or share your email. I won't send you spam. You can unsubscribe at any time.</p>
                    <script async data-uid='b00e0e63dc' src='https://trevordmiller.ck.page/b00e0e63dc/index.js'></script>
                </footer>
              </body>
        </html>
    ",
        &html
    )
}
