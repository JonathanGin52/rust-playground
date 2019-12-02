fn main() {
    println!("{}", count_primes(10));
}

pub fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0
    }
    let n = n as usize;
    let mut primes: Vec<bool> = vec![true; n];
    primes[0] = false;
    primes[1] = false;

    for i in 2..(n as f32).sqrt() as i32 {
        let i = i as usize;
        if primes[i] {
            for j in (i*i..n).step_by(i) {
                let j = j as usize;
                primes[j] = false;
            }
        }
    }
    
    primes.iter().filter(|&n| *n).count() as i32
}
