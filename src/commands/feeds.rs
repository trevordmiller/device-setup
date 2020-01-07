use std::io;

pub fn next() {
    println!("Following feeds so that I'm aware of industry changes.");

    // Next top filtered items from RSS:
    // https://www.reddit.com/r/programming/top/?t=month
    // https://brew.sh/blog
    // https://www.reddit.com/r/commandline/top/?t=month
    // https://www.reddit.com/r/git/top/?t=month
    // https://www.vim.org/news/news.php
    // https://www.reddit.com/r/vim/top/?t=month
    // https://www.reddit.com/r/javascript/top/?t=month
    // https://blog.rust-lang.org
    // https://www.reddit.com/r/rust/top/?t=month
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
