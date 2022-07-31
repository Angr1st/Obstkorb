use core::fmt::Formatter;
use std::str::ParseBoolError;

fn main () -> Result<(),Errors> {
    println!("> Wie viele Obstkörbe möchten Sie?");

    let mut input_buffer = String::new();
    read_answer(&mut input_buffer)?;   
    let korb_anzahl = input_buffer.trim().parse::<usize>()?; 
    let mut körbe = Vec::with_capacity(korb_anzahl);
    for _ in 0..korb_anzahl {
        let mut korb_name = String::new();
        println!("> Wie soll dein Korb heißen?");
        read_answer(&mut korb_name)?;

        let mut fruits = Vec::new();
       
        ask_for_fruit(&mut input_buffer, &mut fruits,Apfel)?;
        ask_for_fruit(&mut input_buffer, &mut fruits,Banane)?;
        ask_for_fruit(&mut input_buffer, &mut fruits,Melone)?;


        let new_korb = Obstkorb {
            name : korb_name.trim().to_string(),
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

fn ask_for_fruit(buffer: &mut  String, fruits: &mut Vec<Box<dyn Fruit>>, fruit: impl Fruit) -> Result<(),Errors> {

    let fruit_name = fruit.plural_name();

    println!("> Soll der Korb {} enthalten?", fruit_name);
        read_answer(buffer)?;
        let has_fruit = ja_oder_nein(&buffer.trim())?;
        if has_fruit {
            println!("> Wie viele {} soll dein Korb enthalten?", fruit_name);
            read_answer(buffer)?;
            let fruit_amount = buffer.trim().parse::<usize>()?;

            for _ in 0..fruit_amount {
                fruits.push(fruit.boxed_new());
            }            
        }

    Ok(())
}

fn ja_oder_nein(answer:&str) -> Result<bool,ParseBoolError> {
    match answer {
        "ja" | "Ja" => Ok(true),
        "nein" | "Nein" => Ok(false),
        x => x.parse::<bool>()
    }
}

fn read_answer(buffer: &mut  String) -> std::io::Result<()> {
    buffer.clear();
    let stdin = std::io::stdin();

    stdin.read_line(buffer)?;
    Ok(())
}

trait Fruit {
    fn name(&self) -> &'static str;
    fn plural_name(&self) -> &'static str;
    fn boxed_new(&self) -> Box<dyn Fruit>;
}

struct Apfel;

impl Fruit for Apfel {
    fn name(&self) -> &'static str {
        "Apfel"
    }

    fn plural_name(&self) -> &'static str {
        "Äpfel"
    }

    fn boxed_new(&self) -> Box<dyn Fruit> {
        let apfel = Apfel;
        Box::new(apfel)
    }
}

struct Banane;

impl Fruit for Banane {
    fn name(&self) -> &'static str {
        "Banane"
    }

    fn plural_name(&self) -> &'static str {
        "Bananen"
    }

    fn boxed_new(&self) -> Box<dyn Fruit> {
        let banane = Banane;
        Box::new(banane)
    }
}


struct Melone;

impl Fruit for Melone {
    fn name(&self) -> &'static str {
        "Melone"
    }

    fn plural_name(&self) -> &'static str {
        "Melonen"
    }

    fn boxed_new(&self) -> Box<dyn Fruit> {
        let melone = Melone;
        Box::new(melone)
    }
}

struct ObstkorbContent 
{
    fruit:Box<dyn Fruit>,
    amount:usize
}

impl ObstkorbContent {
    fn new(fruit:Box<dyn Fruit>) -> ObstkorbContent {
        ObstkorbContent { fruit, amount: 1 }
    }

    fn increment(&mut self, other: &ObstkorbContent) -> bool {
        if self.fruit.name() == other.fruit.name() {
            self.amount = self.amount + 1;
            true
        }
        else {
            false
        }
    }

    fn fold(acc:&mut Vec<Self>, oc: Self) -> &mut Vec<Self> {
        if acc.is_empty() {
            acc.push(oc);
            acc
        }
        else {
            let mut last = acc.pop().expect("Es sollte mindestens ein Element im Vec sein!");
            let is_same_fruit = last.increment(&oc);
            if is_same_fruit {
                acc.push(last);
            }
            else {
                acc.push(last);
                acc.push(oc);
            }
            acc
        }
    } 

    fn output(&self) {
        if self.amount == 1 {
            println!("{} {}", 1, self.fruit.name())
        }
        else {
            println!("{} {}", self.amount, self.fruit.plural_name())
        }
    }
}

struct Obstkorb
{
    name:String,
    fruits:Vec<Box<dyn Fruit>>
}

impl Obstkorb {
    fn output_name(&self) {
        println!("Der Name des Korbs ist: {}",self.name);
    }

    fn output_contents(&self) {
        let mut result_vec: Vec<ObstkorbContent> = Vec::new();
        

        if self.fruits.is_empty() {
            println!("Der Korb ist leer!")
        }
        else {
            println!("Der Korb enthält:");

            self
            .fruits
            .iter()
            .map(|fruit|ObstkorbContent::new(fruit.boxed_new()))
            .fold(&mut result_vec,ObstkorbContent::fold)
            .into_iter()
            .for_each(|oc| oc.output())
        }
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