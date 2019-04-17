use std::io;

fn main() {
    let a = get_input("Please input a:");
    let b = get_input("Please input b:");
    let c = get_input("Please input c:");

    let radicand = 4.0 * a.powi(2) * b.powi(2) - (a.powi(2) + b.powi(2) - c.powi(2)).powi(2);

    let answer = 0.25 * radicand.sqrt();
    println!("The area of the triangle is {}.", answer);
}

fn get_input(prompt: &str) -> f64 {
    loop {
        let mut value = String::new();
        println!("{}", prompt);

        io::stdin().read_line(&mut value)
            .expect("Failed to read line");

        match value.trim().parse() {
            Ok(num) => return num,
            Err(e) => println!("Error: {} \"{}\"", e, value.trim()),
        }
    }
}
