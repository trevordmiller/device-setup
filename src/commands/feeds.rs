use super::printing;

pub fn next() {
    printing::heading("Follow feeds so that I'm aware of industry changes.");

    printing::subheading("Open the top recent items in each feed");

    printing::subheading("Programming");
    printing::list(vec![
        "https://www.reddit.com/r/programming/top/?t=month",
        "https://github.com/trending?since=monthly",
    ]);

    printing::subheading("Unix");
    printing::list(vec![
        "https://brew.sh/blog",
        "https://www.reddit.com/r/commandline/top/?t=month",
    ]);

    printing::subheading("Vim");
    printing::list(vec![
        "https://www.vim.org/news/news.php",
        "https://www.reddit.com/r/vim/top/?t=month",
    ]);

    printing::subheading("Git");
    printing::list(vec!["https://www.reddit.com/r/git/top/?t=month"]);

    printing::subheading("Rust");
    printing::list(vec![
        "https://blog.rust-lang.org",
        "https://www.reddit.com/r/rust/top/?t=month",
    ]);

    printing::subheading("JavaScript");
    printing::list(vec![
        "https://www.reddit.com/r/javascript/top/?t=month",
        "https://nodejs.org/en/blog/",
        "https://www.reddit.com/r/node/top/?t=month",
        "https://reactjs.org/blog/all.html",
        "https://nextjs.org/blog",
    ]);

    printing::pause();
}
