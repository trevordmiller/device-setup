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
        home_file.contains("My resources I write to help me remember what I learn."),
        true
    );

    let writing_file: String = fs::read_to_string("./build/cicd-reference/index.html")
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        writing_file.contains("My reference sheet for automation with CI/CD."),
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
    assert_eq!(rss_file.contains("<item><title>CI/CD reference</title><link>https://trevordmiller.com/cicd-reference/</link><description><![CDATA[My reference sheet for automation with CI/CD.]]></description><author>Trevor D. Miller</author></item>"), true);
}
