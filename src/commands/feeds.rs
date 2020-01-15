use crate::utils::printing;

pub fn next() {
    printing::subheading("Open the top recent items in each feed");

    printing::subheading("Programming");
    printing::list(vec![
        "https://github.com/trending?since=monthly",
        "https://www.reddit.com/r/programming/top/?t=month",
        "https://hackernewsletter.com/issues/",
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
    printing::list(vec![
        "https://github.com/git/git/releases",
        "https://www.reddit.com/r/git/top/?t=month",
    ]);

    printing::subheading("Rust");
    printing::list(vec![
        "https://blog.rust-lang.org",
        "https://www.reddit.com/r/rust/top/?t=month",
    ]);

    printing::subheading("JavaScript");
    printing::list(vec![
        "https://nodejs.org/en/blog/",
        "https://reactjs.org/blog/all.html",
        "https://nextjs.org/blog",
        "https://www.reddit.com/r/node/top/?t=month",
        "https://www.reddit.com/r/javascript/top/?t=month",
    ]);

    printing::pause();
}
