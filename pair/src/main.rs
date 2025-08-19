struct Par<'a, T> {
    primeiro: &'a T,
    segundo: &'a T,
}

impl<'a, T> Par<'a, T> {
    fn novo(x: &'a T, y: &'a T) -> Self {
        Par {
            primeiro: x,
            segundo: y,
        }
    }

    fn primeiro(&self) -> &'a T {
        self.primeiro
    }
    fn segundo(&self) -> &'a T {
        self.segundo
    }
}

fn main() {
    let p = Par::novo(&2, &3);
    println!("primeiro {}", p.primeiro());
    println!("segundo {}", p.segundo());
}
