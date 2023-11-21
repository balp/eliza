use eliza::Eliza;
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, man!");
    let mut answer = String::new();
    let doctor_script = include_str!("../1966_01_CACM_article_doctor_script.eliza");
    let eliza = Eliza::new(doctor_script);
    loop {
        answer.clear();
        print!(": ");
        io::stdout().flush().expect("No flush");
        io::stdin().read_line(&mut answer).expect("Ooops");
        if answer.trim().len() == 0 { return; }
        println!();
        println!(":{}:", answer.trim());
        println!("<{}>", eliza.response(answer.trim()));
    }
}
