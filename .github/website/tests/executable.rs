use assert_cmd::Command;
use std::fs;

#[test]
fn it_creates_website_build_directory() {
    let mut cmd = Command::cargo_bin("trevordmiller_website").unwrap();
    cmd.assert().success();

    let home_file: String = fs::read_to_string("./build/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        home_file.contains("My posts about software development."),
        true
    );

    let post_file: String = fs::read_to_string("./build/programming-principles/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        post_file.contains("My guiding principles for software development in general."),
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
    assert_eq!(rss_file.contains("<item><title>Programming principles</title><link>https://trevordmiller.com/programming-principles/</link><description><![CDATA[My guiding principles for software development in general.]]></description><author>Trevor D. Miller</author></item>"), true);
}
