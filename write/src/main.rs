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

                if route == "index" {
                    paths::create_file(
                        &paths::build().join("index.html"),
                        &markdown_to_html(&markdown_file_contents),
                    );
                } else {
                    paths::create_dir(&paths::build().join(route));
                    paths::create_file(
                        &paths::build().join(route).join("index.html"),
                        &markdown_to_html(&markdown_file_contents),
                    );
                }
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}

fn markdown_to_html(markdown: &str) -> std::string::String {
    let css = r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Helvetica Neue", Helvetica, Arial, Verdana, sans-serif;
            color: #333333;
            max-width: 80ch;
            margin: 0 auto;
            padding: 1rem;
        }
        header a {
            font-size: 1rem;
            font-family: "Courier New", Courier, monospace;
            font-weight: normal;
            text-decoration: none;
            color: #333333;
            display: flex;
            align-items: center;
        }
        header a::before {
            content: ">";
            margin: 0 1ch 0 0;
        }
        header a::after {
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
        li {
            margin-bottom: 0.5rem;
        }
        code {
            overflow: auto;
            display: block;
            background: #f5f5f5;
            padding: 1rem;
        }
        label, input {
            font-size: 1rem;
            display: block;
            margin-bottom: 1rem;
        }
        input[type=submit] {
            font-size: 1rem;
            cursor: pointer;
            border: 0 none;
            -moz-appearance: none;
            -webkit-appearance: none;
            background: #333333; 
            color: #ffffff;
            padding: 0.5rem; 
        }
    "#;

    let parser = Parser::new_ext(markdown, Options::empty());
    let mut html = String::new();
    html::push_html(&mut html, parser);

    format!("
        <!DOCTYPE html>
        <html lang='en-US'>
            <head>
                <title>trevordmiller.com</title>
                <meta name='author' content='Trevor D. Miller'>
                <meta name='description' content='Personal website.'>
                <meta charset='utf-8'>
                <meta name='viewport' content='width=device-width, initial-scale=1'>
                <style type='text/css'>
                    {}
                </style>
            </head>
            <body>
                <header>
                    <nav>
                        <h1><a href='/'>trevordmiller</a></h1>
                    </nav>
                </header>
                <main>
                    {}
                </main>
                <footer>
                    <h2>Join my newsletter</h2>
                    <form action='https://trevordmiller.us10.list-manage.com/subscribe/post?u=91fe993c2d93cde48679d6826&amp;id=f7f097d693' method='post' id='mc-embedded-subscribe-form' name='mc-embedded-subscribe-form'>
                        <label for='mce-EMAIL'>Email address:</label>
                        <input type='email' required placeholder='Your email' value='' name='EMAIL' id='mce-EMAIL'>
                        <!-- real people should not fill this in and expect good things - do not remove this or risk form bot signups-->
                        <div style='position: absolute; left: -5000px;' aria-hidden='true'><input type='text' name='b_91fe993c2d93cde48679d6826_f7f097d693' tabindex='-1' value=''></div>
                        <input type='submit' value='Subscribe' placeholder='you@somewhere.com' name='subscribe' id='mc-embedded-subscribe'>
                    </form>
                </footer>
            </body>
        </html>
    ",
        css,
        &html
    )
}
