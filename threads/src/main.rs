use std::sync::Arc;
use std::thread;

fn main() {
    let v = vec![10; 10];
    let v = Arc::new(v);
    let len = v.len();
    let nthreads = 4;
    let chunk_size = len / nthreads;
    let mut threads = Vec::new();
    for i in 0..nthreads {
        let vstart = i * chunk_size;
        let vend = if i == nthreads - 1 {
            len
        } else {
            (i + 1) * chunk_size
        };
        //let slice = v[vstart..vend].to_vec();
        let vref = Arc::clone(&v);
        threads.push(thread::spawn(move || {
            let slice = &vref[vstart..vend];
            let sum: i32 = slice.iter().sum();
            sum
        }));
    }
    let mut sum = 0;
    for t in threads {
        sum += t.join().unwrap();
    }
    println!("{}", sum);
}
