use std::io;

fn main() -> io::Result<()> {
    println!("What name has your fruit basket?");
    let mut buffer_name = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer_name)?;
    
    let apple = if fruit_question(&stdin, String::from("apple")) {
            Some(Apfel)
        } else {
            None
        };

    let korb = Obstkorb {
            name : buffer_name,
            apfel : apple,
            ..Default::default()
        };

    Ok(())
}

fn fruit_question(stdin: &io::Stdin, frucht:String) -> bool {
    println!("Does your basket contain an {}?", frucht);
    let mut buffer_fruit = String::new();
    stdin.read_line(&mut buffer_fruit).expect("Should be able to read from stdin!");

    buffer_fruit.trim().parse::<bool>().expect("Should be true or false")
}

struct Apfel;
struct Banane;
struct Melone;

#[derive(Default)]
struct Obstkorb {
    name : String,
    apfel : Option<Apfel>,
    banane : Option<Banane>,
    melone : Option<Melone>
}
