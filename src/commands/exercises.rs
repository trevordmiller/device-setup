use std::io;

pub fn next() {
    println!("Completing an exercise so that I gain experience solving problems.");

    // Next incomplete exercise from:
    // exercises/
    // README.md
    // cargo verification commands
    // Compare my solution with the example solution
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
