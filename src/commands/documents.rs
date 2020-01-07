use super::progress;

pub fn next() {
    println!("Reading a document so that I increase what I know.");

    // Next incomplete document in list:
    //
    // Unix
    // The Linux Command Line: http://linuxcommand.org/tlcl.php
    //
    // Vim
    // Practial Vim: https://pragprog.com/book/dnvim2/practical-vim-second-edition
    //
    // Rust
    // The Rust Programming Language: https://doc.rust-lang.org/book/
    // Rust by Example: https://doc.rust-lang.org/stable/rust-by-example/
    // Rust Standard Library: https://doc.rust-lang.org/std/index.html
    // Rust Command Line Book: https://rust-lang-nursery.github.io/cli-wg/
    // Rust WebAssembly Book: https://rust-lang-nursery.github.io/cli-wg/

    progress::pause();
}
