use eliza::Eliza;
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, man!");
    let mut answer = String::new();
    let eliza = Eliza {};
    loop {
        answer.clear();
        print!(": ");
        io::stdout().flush().expect("No flush");
        io::stdin().read_line(&mut answer).expect("Ooops");
        println!();
        println!(":{}:", answer.trim());
        println!("<{}>", eliza.process(answer.trim()));
    }
}
