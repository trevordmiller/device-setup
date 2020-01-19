use crate::utils::printing;

pub fn next() {
    printing::subheading("Write a note so that I solidify the most important things I've learned.");
    printing::list(vec![
        "vim ./src/notes/{category}/{kebab-case-topic}.md",
        "Use a title in the format '# {Topic} in {Category}'",
    ]);
    printing::pause();
}
