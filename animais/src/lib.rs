trait Animal {
    fn fazer_barulho(&self) -> String;
}

pub struct Cachorro {}
pub struct Gato {}

impl Animal for Cachorro {
    fn fazer_barulho(&self) -> String {
        "au au".to_string()
    }
}

impl Animal for Gato {
    fn fazer_barulho(&self) -> String {
        "miau".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cachorro() {
        let cao = Cachorro {};
        assert_eq!(cao.fazer_barulho(), "au au".to_string());
    }

    #[test]
    fn test_gato() {
        let gato = Gato {};
        assert_eq!(gato.fazer_barulho(), "miau".to_string());
    }
}
