use crate::utils::printing;

pub fn next() {
    printing::subheading("Read a document so that I increase what I know.");
    printing::list(vec![
        "[X] http://linuxcommand.org/tlcl.php",
        "[X] https://pragprog.com/book/dnvim2/practical-vim-second-edition",
        "[ ] https://doc.rust-lang.org/book/",
        "[ ] https://doc.rust-lang.org/stable/rust-by-example/",
        "[ ] https://doc.rust-lang.org/std/index.html",
        "[ ] https://rust-lang-nursery.github.io/cli-wg/",
        "[ ] https://rustwasm.github.io/docs/book/",
        "[ ] https://archive.org/details/MythicalManMonth",
        "[ ] http://www.charlespetzold.com/code/",
        "[ ] https://pragprog.com/book/tpp/the-pragmatic-programmer",
        "[ ] http://codersatwork.com",
        "[ ] https://mitpress.mit.edu/books/introduction-algorithms"
    ]);
    printing::pause();
}
