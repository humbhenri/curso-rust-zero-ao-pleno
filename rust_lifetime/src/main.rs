use rust_lifetime::maior_string;

fn main() {
    let strings = vec!["a", "abc", "Humberto"];
    let maior_s = maior_string(&strings);
    println!("Maior string = {:?}", maior_s);
}
