use std::io;

pub fn heading(content: &str) {
    padding();
    padding();
    println!("===========================================================================================================");
    println!("{}", content);
    println!("===========================================================================================================");
}

pub fn subheading(content: &str) {
    padding();
    println!("-----------------------------------------------------------------------------------------------------------");
    println!("{}", content);
    println!("-----------------------------------------------------------------------------------------------------------");
}

pub fn list(items: Vec<&str>) {
    padding();
    items.iter().for_each(|content| println!("- {}", content));
}

pub fn pause() {
    divide();
    loop {
        padding();
        println!("Enter 'Done' when ready to continue:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }

        if input.contains("Done") {
            break;
        } else {
            continue;
        }
    }
}

fn divide() {
    padding();
    padding();
    println!("...........................................................................................................");
}

fn padding() {
    println!();
}
