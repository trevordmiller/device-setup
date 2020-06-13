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
    let css = r#"
        html {
            font-family: -apple-system, BlinkMacSystemFont, "Helvetica Neue", Helvetica, Arial, Verdana, sans-serif;
            color: #333333;
        }
        body {
            margin: 0;
        }
        header {
            background: #f5f5f5;
            border-bottom: 1px solid #d3d3d3;
        }
        header nav {
            max-width: 80ch;
            margin: 0 auto;
        }
        header a {
            padding: 1rem;
            font-family: "Courier New", Courier, monospace;
            text-decoration: none;
            color: #333333;
            display: block;
        }
        header a:first-of-type {
            display: flex;
            align-items: center;
        }
        header a:first-of-type::before {
            content: ">";
            margin: 0 1ch 0 0;
        }
        header a:first-of-type::after {
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
        @media screen and (min-width: 80ch) {
            header {
                position: sticky;
                top: 0;
            }
            header nav {
                display: flex;
                align-items: center;
            }
            header a:first-of-type {
                flex: 1;
            }
        }
        main {
            max-width: 80ch;
            margin: 0 auto;
            padding: 1rem;
        }
        li {
            margin-bottom: 1rem;
        }
        pre code {
            overflow: auto;
            display: block;
            padding: 1rem;
            background: #f5f5f5;
            border-radius: 3px;
        }
        footer {
            max-width: 80ch;
            margin: 2rem auto 0 auto;
            padding: 2rem 1rem 1rem 1rem;
            border-top: 1px solid #d3d3d3;
        }
        label, input {
            display: block;
            font-size: 1rem;
            margin-bottom: 1rem;
        }
        input[type=submit] {
            -moz-appearance: none;
            -webkit-appearance: none;
            padding: 1rem; 
            background: #333333; 
            color: #ffffff;
            font-size: 1rem;
            border: 0 none;
            cursor: pointer;
            border-radius: 3px; 
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
                    <form action='https://trevordmiller.us10.list-manage.com/subscribe/post?u=91fe993c2d93cde48679d6826&amp;id=f7f097d693' method='post' id='mc-embedded-subscribe-form' name='mc-embedded-subscribe-form'>
                        <label for='mce-EMAIL'>Email address:</label>
                        <input type='email' required value='' name='EMAIL' id='mce-EMAIL'>
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
