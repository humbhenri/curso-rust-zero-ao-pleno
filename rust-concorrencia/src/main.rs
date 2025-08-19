use rayon::prelude::*;
use std::time::Instant;

fn is_primo(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f32).sqrt() as u32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let numbers: Vec<u32> = (2..=1999).collect();
    let s1 = Instant::now();
    let c1 = numbers.iter().filter(|&n| is_primo(*n)).count();
    let d1 = s1.elapsed();
    println!("Duração sequencial {:?}, quantidade de primos {}", d1, c1);

    let s2 = Instant::now();
    let c2 = numbers
        .iter()
        .par_bridge()
        .filter(|&n| is_primo(*n))
        .count();
    let d2 = s2.elapsed();
    println!("Duração paralela {:?}, quantidade de primos {}", d2, c2);
}
