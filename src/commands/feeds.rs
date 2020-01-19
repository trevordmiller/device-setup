use crate::utils::printing;

pub fn next() {
    printing::subheading("Follow feeds so that I'm aware of industry changes.");
    printing::list(vec![
        "https://github.com/trending?since=monthly",
        "https://www.reddit.com/r/programming/top/?t=month",
        "https://brew.sh/blog",
        "https://www.reddit.com/r/commandline/top/?t=month",
        "https://www.vim.org/news/news.php",
        "https://www.reddit.com/r/vim/top/?t=month",
        "https://github.com/git/git/releases",
        "https://www.reddit.com/r/git/top/?t=month",
        "https://blog.rust-lang.org",
        "https://www.reddit.com/r/rust/top/?t=month",
        "https://nodejs.org/en/blog/",
        "https://reactjs.org/blog/all.html",
        "https://nextjs.org/blog",
        "https://www.reddit.com/r/node/top/?t=month",
        "https://www.reddit.com/r/javascript/top/?t=month",
    ]);
    printing::pause();
}
