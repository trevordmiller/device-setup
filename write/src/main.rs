use rss::ChannelBuilder;
use rss::Item;
use std::fs;

mod decoration;
mod markdown;
mod paths;
mod printing;

fn main() {
    printing::heading("Creating website build directory.");

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
    let mut rss_items = Vec::new();

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
                    None => panic!(
                        "Cannot find a break between title and description in {}.",
                        &route
                    ),
                };

                let description = match lines.next() {
                    Some(description) => description.replace("_", ""),
                    None => panic!("Cannot find a description in {}.", &route),
                };

                if route == "index" {
                    paths::create_file(
                        &paths::build().join("index.html"),
                        &markdown::to_html(&markdown_file_contents, &title, &description),
                    );
                } else {
                    paths::create_dir(&paths::build().join(route));
                    paths::create_file(
                        &paths::build().join(route).join("index.html"),
                        &markdown::to_html(&markdown_file_contents, &title, &description),
                    );
                }

                let is_individual_article = route != "index"
                    && route != "about"
                    && route != "resume"
                    && route != "projects";

                if is_individual_article {
                    let mut rss_item = Item::default();
                    rss_item.set_link(format!("https://trevordmiller.com/{}/", &route));
                    rss_item.set_title(title);
                    rss_item.set_description(description);
                    rss_items.push(rss_item)
                }
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    let channel = match ChannelBuilder::default()
        .language("en-us".to_string())
        .link("https://trevordmiller.com")
        .title("trevordmiller")
        .description("trevordmiller RSS feed.")
        .items(rss_items)
        .build()
    {
        Ok(channel) => channel.to_string(),
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    paths::create_file(&paths::build().join("rss.xml"), &channel);
}
