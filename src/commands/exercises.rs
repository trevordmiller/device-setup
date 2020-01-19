use crate::utils::printing;

pub fn next() {
    printing::subheading("Complete an exercise so that I gain experience solving problems.");
    printing::list(vec![
        "cd ./src/exercises/{category}/{exercise}",
        "Read the README.md",
        "Make verification commands pass",
        "Compare my solution with the example solution",
    ]);
    printing::pause();
}
