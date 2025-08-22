#[derive(Debug, Copy, Clone)]
pub struct MyResult<T, E>(Result<T, E>);

impl<T, E> MyResult<T, E> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where F: FnOnce() -> T {
        match self.0 {
            Ok(x) => x,
            Err(_e) => f()
        }
    }
}

fn main() {
    let fallback_value = || 0;
    let r: MyResult<_, String> = MyResult(Ok(10));
    println!("{}", r.unwrap_or_else(fallback_value));
    let t: MyResult<_, String> = MyResult(Err("erro".to_string()));
    println!("{}", t.unwrap_or_else(fallback_value));
}
