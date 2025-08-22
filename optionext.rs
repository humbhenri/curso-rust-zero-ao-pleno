trait OptionExt<T, F> {
    fn unwrap_or_else(&self, f: F) -> T
    where
        F: FnOnce() -> T;
    fn map(&self, f: F) -> Option<T>
    where
        F: FnOnce(T) -> T;
}

impl<T: Clone, F> OptionExt<T, F> for Option<T> {
    fn unwrap_or_else(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x.clone(),
            None => f(),
        }
    }

    fn map(&self, f: F) -> Option<T>
    where
        F: FnOnce(T) -> T,
    {
        match self {
            Some(x) => Some(f(x.clone())),
            None => None,
        }
    }
}

fn main() {
    let closure = || 2;
    let optional_number: Option<i32> = Some(42);
    println!("valor {}", optional_number.unwrap_or_else(closure));
    println!("valor {}", None.unwrap_or_else(closure));
    let x = optional_number.map(|x| x * x);
    print!("x = {}", x.unwrap_or_else(closure));
}
