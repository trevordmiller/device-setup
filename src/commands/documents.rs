use super::printing;

pub fn next() {
    printing::heading("Read a document so that I increase what I know.");

    printing::subheading("Read the next incomplete document");

    printing::subheading("Unix");
    printing::list(vec!["http://linuxcommand.org/tlcl.php"]);

    printing::subheading("Vim");
    printing::list(vec![
        "https://pragprog.com/book/dnvim2/practical-vim-second-edition",
    ]);

    printing::subheading("Rust");
    printing::list(vec![
        "https://doc.rust-lang.org/book/",
        "https://doc.rust-lang.org/stable/rust-by-example/",
        "https://doc.rust-lang.org/std/index.html",
        "https://rust-lang-nursery.github.io/cli-wg/",
        "https://rustwasm.github.io/docs/book/",
    ]);

    printing::subheading("JavaScript");
    printing::list(vec!["https://developer.mozilla.org/en-US/docs/WebAssembly"]);

    printing::pause();
}
