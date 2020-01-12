use super::printing;

pub fn next() {
    printing::heading("Complete an exercise so that I gain experience solving problems.");

    printing::subheading("Complete the next incomplete exercise");

    printing::list(vec![
        "cd exercises/{category}/{exercise}",
        "Read the README.md",
        "Make verification commands pass",
        "Compare my solution with the example solution",
    ]);

    printing::pause();
}
