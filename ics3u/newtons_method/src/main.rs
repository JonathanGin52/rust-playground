use std::io;
use std::process;
use std::fmt;

struct Term {
    coefficient: f64,
    degree: i32,
}

impl Term {
    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * x.powi(self.degree)
    }

    fn derive(&self) -> Term {
        if self.degree == 0 {
            return Term {
                coefficient: 0_f64,
                degree: 0_i32,
            }
        }
        Term {
            coefficient: self.coefficient * self.degree as f64,
            degree: self.degree - 1,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}x^{}", self.coefficient, self.degree)
    }
}

fn main() {
    println!("Enter a polynomial function:");
    let mut function = String::new();

    io::stdin().read_line(&mut function)
        .expect("Failed to read line");

    let function = parse_polynomial(function);

    if let Some(root) = newtons_method(function) {
        println!("An estimated root is: {}", root);
    } else {
        println!("No root found.");
    }
}

fn parse_polynomial(polynomial: String) -> Vec<Term> {
    polynomial.replace("-", "+-")
        .split('+')
        .map(|s| {
            let term = String::from(s.trim());
            let coefficient;
            let mut degree = 1_i32;

            if let Some(index) = term.find('x') {
                coefficient = term.chars()
                    .take(index)
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if let Some(index) = term.find('^') {
                    degree = term.chars()
                        .skip(index + 1)
                        .take(term.len())
                        .collect::<String>()
                        .parse()
                        .unwrap();
                }
            } else {
                coefficient = term.parse().unwrap();
                degree = 0;
            }

            Term {
                coefficient,
                degree,
            }
        })
        .collect()
}

fn newtons_method(polynomial: Vec<Term>) -> Option<f64> {
    println!("Enter an initial guess:");
    const DELTA: f64 = 0.0001;
    let mut root = String::new();

    io::stdin().read_line(&mut root)
        .expect("Failed to read line");

    let mut root = match root.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(1),
    };

    println!("Running Newton's method with an initial guess of: {}", root);
    let derivative: Vec<Term> = polynomial.iter()
        .map(|term| term.derive())
        .collect();
    for _ in 1..10000 {
        let new_root = root - evaluate_polynomial(&polynomial, root) / evaluate_polynomial(&derivative, root);
        if (new_root - root).abs() < DELTA {
            return Some(root)
        }
        root = new_root;
    }
    None
}

fn evaluate_polynomial(polynomial: &Vec<Term>, x: f64) -> f64 {
    polynomial.iter()
        .map(|t| t.evaluate(x))
        .sum()
}
