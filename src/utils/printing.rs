use std::io;

pub fn heading(content: &str) {
    clear();
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

pub fn progress(content: String) {
    println!("{}", content);
}

pub fn info(content: String) {
    println!("{}", content);
}

pub fn error(content: String) {
    println!("{}", content);
}

pub fn list(items: Vec<&str>) {
    padding();
    items.iter().for_each(|content| println!("- {}", content));
}

pub fn pause() {
    padding();
    padding();
    padding();
    println!("...");
    loop {
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

pub fn clear() {
    print!("{}[2J", 27 as char);
}

fn padding() {
    println!();
}
