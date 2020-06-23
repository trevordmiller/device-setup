use assert_cmd::Command;
use std::fs;

#[test]
fn it_creates_website_build_directory() {
    let mut cmd = Command::cargo_bin("write").unwrap();
    cmd.assert().success();

    let articles_file: String = fs::read_to_string("./build/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        articles_file.contains("My posts about software development."),
        true
    );

    let article_file: String = fs::read_to_string("./build/standard-vim/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        article_file.contains("Using Vim without customizations."),
        true
    );

    let about_file: String = fs::read_to_string("./build/about/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(about_file.contains("My personal background."), true);

    let resume_file: String = fs::read_to_string("./build/resume/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(resume_file.contains("My professional background."), true);

    let projects_file: String = fs::read_to_string("./build/projects/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(projects_file.contains("My open source projects."), true);

    let rss_file: String = fs::read_to_string("./build/rss.xml")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(rss_file.contains("<item><title>Standard Vim</title><link>https://trevordmiller.com/standard-vim/</link><description><![CDATA[Using Vim without customizations.]]></description></item>"), true);

    let cname_file: String = fs::read_to_string("./build/CNAME")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(cname_file, "trevordmiller.com");
}
