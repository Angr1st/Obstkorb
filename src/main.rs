use core::fmt::Formatter;

fn main () -> Result<(),Errors> {
    println!("Wie viele Obstkörbe möchten Sie?");

    let mut input_buffer = String::new();
    read_answer(&mut input_buffer)?;   
    let korb_anzahl = input_buffer.trim().parse::<usize>()?; 
    let mut körbe = Vec::with_capacity(korb_anzahl);
    for _ in 0..korb_anzahl {
        let mut korb_name = String::new();
        println!("Wie soll dein Korb heißen?");
        read_answer(&mut korb_name)?;

        let mut fruits = Vec::new();

        println!("Soll der Korb Äpfel enthalten?");
        read_answer(&mut input_buffer)?;
        let has_apfel = input_buffer.trim().parse::<bool>()?;
        if has_apfel {
            println!("Wie viele Äpfel soll dein Korb enthalten?");
            read_answer(&mut input_buffer)?;
            let apfel_amount = input_buffer.trim().parse::<usize>()?;
            let mut apfels = Vec::with_capacity(apfel_amount);
            for _ in 0..apfel_amount {
                let apfel = Apfel;
                let box_apfel : Box<dyn Fruit> = Box::new(apfel);
                apfels.push(box_apfel);
            }

            fruits.append(&mut apfels);
        }
       
        ask_for_fruit(&mut input_buffer, &mut fruits,"Äpfel",Apfel::new_boxed)?;

        let new_korb = Obstkorb {
            name : korb_name,
            fruits
        };
        körbe.push(new_korb);
    }

    for korb in körbe {
        korb.output_name();
        korb.output_contents();
    }

    Ok(())
}

fn ask_for_fruit(buffer: &mut  String, fruits: &mut Vec<Box<dyn Fruit>>, fruit_name: &str, create_fruit: fn() -> Box<dyn Fruit>) -> Result<(),Errors> {

    println!("Soll der Korb {} enthalten?", fruit_name);
        read_answer(buffer)?;
        let has_fruit = buffer.trim().parse::<bool>()?;
        if has_fruit {
            println!("Wie viele {} soll dein Korb enthalten?", fruit_name);
            read_answer(buffer)?;
            let fruit_amount = buffer.trim().parse::<usize>()?;

            for _ in 0..fruit_amount {
                fruits.push(create_fruit());
            }            
        }

    Ok(())
}

fn read_answer(buffer: &mut  String) -> std::io::Result<()> {
    buffer.clear();
    let stdin = std::io::stdin();

    stdin.read_line(buffer)?;
    Ok(())
}

trait Fruit {
    fn name(&self) -> String;
}

struct Apfel;

impl Apfel {
    fn new_boxed() -> Box<dyn Fruit> {
        let apfel = Apfel;
        Box::new(apfel)
    }
}

impl Fruit for Apfel {
    fn name(&self) -> String {
        String::from("Apfel")
    }
}

struct Banane;

impl Banane {
    fn new_boxed() -> Box<dyn Fruit> {
        let banane = Banane;
        Box::new(banane)
    }
}

impl Fruit for Banane {
    fn name(&self) -> String {
        String::from("Banane")
    }
}


struct Melone;

impl Melone {
    fn new_boxed() -> Box<dyn Fruit> {
        let melone = Melone;
        Box::new(melone)
    }
}

impl Fruit for Melone {
    fn name(&self) -> String {
        String::from("Melone")
    }
}


struct Obstkorb
{
    name:String,
    fruits:Vec<Box<dyn Fruit>>
}

impl Obstkorb {
    fn output_name(&self) {
        println!("{}",self.name);
    }

    fn output_contents(&self) {
        println!("Der Korb enthält:");
        self.fruits.iter().for_each(|fruit| println!("{}",fruit.name()));
    }
}

#[derive(Debug)]
enum Errors {
    ParseBool(std::str::ParseBoolError),
    ParseNumber(std::num::ParseIntError),
    IoError(std::io::Error)
}

impl std::convert::From<std::str::ParseBoolError> for Errors {
    fn from(parse_err: std::str::ParseBoolError) -> Self {
        Errors::ParseBool(parse_err)
    }
}

impl std::convert::From<std::num::ParseIntError> for Errors {
    fn from(parse_err: std::num::ParseIntError) -> Self {
        Errors::ParseNumber(parse_err)
    }
}

impl std::convert::From<std::io::Error> for Errors {
    fn from(io_err: std::io::Error) -> Self {
        Errors::IoError(io_err)
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
       match self {
         Errors::ParseBool(p) => write!(f,"{}",p),
         Errors::ParseNumber(p) => write!(f,"{}",p),
         Errors::IoError(i) => write!(f,"{}",i)
       } 
    }
}

impl std::error::Error for Errors {
    
}