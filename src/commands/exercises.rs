use std::io;

pub fn next() {
    println!("Completing an exercise so that I gain experience solving problems.");

    // Next incomplete exercise in:
    // exercises/{category}/{exercise}
    // README.md
    // {Verification commands specific to category}
    // example.{category specific filetype} solution
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
