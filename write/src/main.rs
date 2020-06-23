use pulldown_cmark::{html, Options, Parser};
use std::fs;

mod paths;
mod printing;

fn main() {
    printing::heading("Creating the build directory for my website.");

    printing::subheading("Cleaning the build directory.");
    paths::remove_dir(&paths::build());

    printing::subheading("Generating the build directory.");
    paths::create_dir(&paths::build());
    generate_pages();

    printing::subheading("Configuring the build directory.");
    // Adds a CNAME file for the host (GitHub Pages) and registrar (Hover) to use my custom domain name (trevordmiller.com)
    paths::create_file(&paths::build().join("CNAME"), "trevordmiller.com");
}

fn generate_pages() {
    match fs::read_dir(&paths::pages()) {
        Ok(markdown_files) => {
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

                let mut lines = markdown_file_contents.lines();

                let title = match lines.next() {
                    Some(title) => title.replace("# ", ""),
                    None => panic!("Cannot find a title in {}.", &route),
                };

                match lines.next() {
                    Some(_) => (),
                    None => panic!("Cannot find a break between title and description in {}.", &route),
                };

                let description = match lines.next() {
                    Some(description) => description.replace("_", ""),
                    None => panic!("Cannot find a description in {}.", &route),
                };

                if route == "index" {
                    paths::create_file(
                        &paths::build().join("index.html"),
                        &markdown_to_html(&markdown_file_contents, &title, &description),
                    );
                } else {
                    paths::create_dir(&paths::build().join(route));
                    paths::create_file(
                        &paths::build().join(route).join("index.html"),
                        &markdown_to_html(&markdown_file_contents, &title, &description),
                    );
                }
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}

fn markdown_to_html(markdown: &str, title: &str, description: &str) -> std::string::String {
    let css = r#"
        body {
            max-width: 80ch;
            margin: 0 auto;
            padding: 1rem 1rem 2rem 1rem;
            font-family: "Helvetica Neue", Helvetica, Arial, Verdana, sans-serif;
            color: #333333;
        }
        code {
            overflow: auto;
            display: block;
            background: #f5f5f5;
        }
        nav a, li, code {
            padding: 0.5rem;
        }
        header, code {
            font-family: "Courier New", Courier, monospace;
        }
        main {
            margin-top: 1rem;
            border-top: 1px solid #d3d3d3;
        }
        header, nav {
            display: flex;
            align-items: center;
            flex-wrap: wrap;
        }
        header::before {
            content: ">";
            margin: 0 1ch 0 0;
        }
        header::after {
            content: "";
            margin: 0 0 0 1ch;
            display: inline-block;
            background: #333333;
            width: 1ch;
            height: 1em;
            animation: cursor 800ms infinite;
        }
        @keyframes cursor {
            0% {
                opacity: 0;
            }
            50% {
                opacity: 1;
            }
            to {
                opacity: 0;
            }
        }
    "#;

    let parser = Parser::new_ext(markdown, Options::empty());
    let mut html = String::new();
    html::push_html(&mut html, parser);

    format!("
        <!DOCTYPE html>
        <html lang='en-US'>
            <head>
                <title>trevordmiller | {}</title>
                <meta name='description' content='{}'>
                <meta name='author' content='Trevor D. Miller'>
                <meta charset='utf-8'>
                <meta name='viewport' content='width=device-width, initial-scale=1'>
                <style type='text/css'>
                    {}
                </style>
            </head>
            <body>
                <header>trevordmiller</header>
                <nav>
                    <a href='/'>Articles</a>
                    <a href='/about/'>About</a>
                    <a href='/resume/'>Resume</a>
                    <a href='/projects/'>Projects</a>
                </nav>
                <main>
                    {}
                </main>
            </body>
        </html>
    ",
        &title,
        &description,
        &css,
        &html
    )
}
