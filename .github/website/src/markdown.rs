use pulldown_cmark::{html, Options, Parser};

pub fn to_html(markdown: &str, title: &str, description: &str) -> std::string::String {
    let css = r#"
        body {
            max-width: 80ch;
            margin: 0 auto;
            padding: 1rem 1rem 2rem 1rem;
            font-size: 1rem;
            line-height: 1.5;
        }
        body, code {
            font-family: "Courier New", Courier, monospace;
        }
        code {
            background: whitesmoke;
            padding: 0.05rem 0.5rem;
        }
        pre code {
            overflow-x: auto;
            display: block;
            padding: 1rem;
        }
        nav a {
            padding: 1ch;
        }
        h1, h2 {
            font-weight: normal;
            font-size: 1rem;
            margin-bottom: 1rem;
        }
        h1 {
            text-transform: uppercase;
            margin-top: 0;
        }
        h2 {
            text-decoration: underline;
            margin-top: 2rem;
        }
        main {
            margin-top: 1rem;
            border-top: 1px dashed black;
            padding-top: 2rem;
            padding-bottom: 2rem;
            border-bottom: 1px dashed black;
            margin-bottom: 3rem;
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
            background: black;
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
                <link rel='alternate' type='application/rss+xml' title='trevordmiller RSS feed' href='/rss.xml'>
            </head>
            <body>
                <header>trevordmiller</header>
                <nav>
                    <a href='/'>posts/</a>
                    <a href='/about/'>about/</a>
                    <a href='/resume/'>resume/</a>
                    <a href='/projects/'>projects/</a>
                </nav>
                <main>
                    {}
                </main>
                <footer>
                    <p>If you would like to get updates when I publish new content, join my email list by sending an email to <code>trevordmiller+subscribe<wbr>@groups.io</code> with a subject of <code>subscribe</code>.</p>
                </footer>
            </body>
        </html>
    ",
        &title,
        &description,
        &css,
        &html
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_html_documents() {
        let markdown_file_contents = "- Some list item.";
        let title = "Some title";
        let description = "Some description.";

        let html_document = to_html(&markdown_file_contents, &title, &description);

        assert_eq!(
            html_document.contains("<title>trevordmiller | Some title</title>"),
            true
        );
        assert_eq!(
            html_document.contains("<meta name='description' content='Some description.'>"),
            true
        );
        assert_eq!(
            html_document.contains("font-family: \"Courier New\", Courier, monospace;"),
            true
        );
        assert_eq!(html_document.contains("/rss.xml"), true);
        assert_eq!(html_document.contains("<li>Some list item.</li>"), true);
        assert_eq!(html_document.contains("trevordmiller+subscribe"), true);
    }
}
