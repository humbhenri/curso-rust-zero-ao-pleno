pub struct Repositorio<T> {
    data: Vec<T>,
}

impl<T> Repositorio<T> {
    pub fn nova() -> Self {
        Repositorio { data: Vec::new() }
    }

    pub fn adiciona(&mut self, e: T) {
        self.data.push(e);
    }
}

impl Repositorio<i32> {
    pub fn soma(&self) -> i32 {
        self.data.iter().sum()
    }

    pub fn produto(&self) -> i32 {
        self.data.iter().product()
    }
}

impl Repositorio<f64> {
    pub fn soma(&self) -> f64 {
        self.data.iter().sum()
    }

    pub fn produto(&self) -> f64 {
        self.data.iter().product()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nova() {
        let r: Repositorio<i32> = Repositorio::nova();
        assert!(r.data.is_empty());
    }

    #[test]
    fn test_adiciona() {
        let mut r: Repositorio<i32> = Repositorio::nova();
        r.adiciona(10);
        assert_eq!(r.data[0], 10);
    }

    #[test]
    fn test_soma() {
        let mut r: Repositorio<i32> = Repositorio::nova();
        r.adiciona(10);
        r.adiciona(20);
        assert_eq!(r.soma(), 30);
        assert_eq!(r.produto(), 200);
    }
}
