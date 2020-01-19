use crate::utils::printing;

pub fn next() {
    printing::heading("Write a note so that I solidify the most important things I've learned.");

    printing::subheading("Add a new note");

    printing::list(vec![
        "vim notes/{category}/{kebab-case-topic}.md",
        "Use a title in the format '# {Topic} in {Category}'",
        "Keep contents encapsulated in the markdown file",
    ]);

    printing::pause();
}
