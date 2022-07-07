use std::io;

fn main() -> io::Result<()> {
    println!("What name has your fruit basket?");
    let mut buffer_name = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer_name)?;
    
    println!("Does your basket contain an apple?");
    let mut buffer_apple = String::new();
    stdin.read_line(&mut buffer_apple)?;
    let apple = if buffer_apple.trim().parse::<bool>().expect("Should be true or false") {
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
