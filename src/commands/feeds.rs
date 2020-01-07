use std::io;

pub fn next() {
    println!("Following feeds so that I'm aware of industry changes.");

    // Next top recent items from RSS:
    //
    // Programming
    // Programming subreddit: https://www.reddit.com/r/programming/top/?t=month
    //
    // Unix
    // Homebrew blog: https://brew.sh/blog
    // Command line subreddit: https://www.reddit.com/r/commandline/top/?t=month
    //
    // Vim
    // Vim news: https://www.vim.org/news/news.php
    // Vim subreddit: https://www.reddit.com/r/vim/top/?t=month
    //
    // Git
    // Git subreddit: https://www.reddit.com/r/git/top/?t=month
    //
    // Rust
    // Rust blog: https://blog.rust-lang.org
    // Rust subreddit: https://www.reddit.com/r/rust/top/?t=month
    //
    // JavaScript
    // JavaScript subreddit: https://www.reddit.com/r/javascript/top/?t=month
    loop {
        println!("Enter 'Continue' to continue");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }

        if input.contains("Continue") {
            break;
        } else {
            continue;
        }
    }
}
