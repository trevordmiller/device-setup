use std::io;

pub fn next() {
    println!("Reading a document so that I increase what I know.");

    // Next incomplete document (section / chapter / article / guide / tutorial) from list:
    // - ~~[The Linux Command Line](http://linuxcommand.org/tlcl.php)~~
    // - ~~[Practial Vim](https://pragprog.com/book/dnvim2/practical-vim-second-edition)~~
    // - [The Rust Programming Language](https://doc.rust-lang.org/book/)
    // - [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
    // - [Rust Standard Library](https://doc.rust-lang.org/std/index.html)
    // - [Rust Command Line Book](https://rust-lang-nursery.github.io/cli-wg/)
    // - [Rust WebAssembly Book](https://rust-lang-nursery.github.io/cli-wg/)
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
