use crate::utils::printing;

pub fn next() {
    printing::subheading("Write a note so that I solidify the most important things I've learned.");
    printing::list(vec![
        "The file should be in the format ./src/notes/{category-topic}.md",
        "The first line should be the title in the format '# {Topic} in {Category}'",
    ]);
    printing::pause();
}
