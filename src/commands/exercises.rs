use crate::utils::printing;

pub fn next() {
    printing::subheading("Complete the next incomplete exercise");

    printing::list(vec![
        "cd exercises/{category}/{exercise}",
        "Read the README.md",
        "Make verification commands pass",
        "Compare my solution with the example solution",
    ]);

    printing::pause();
}
