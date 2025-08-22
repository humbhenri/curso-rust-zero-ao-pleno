trait Media {
    fn reproduzir(&self) -> String;
}

pub struct Filme;
pub struct Serie;

impl Media for Filme {
    fn reproduzir(&self) -> String {
        "Filme".to_string()
    }
}

impl Media for Serie {
    fn reproduzir(&self) -> String {
        "Serie".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filme() {
        let filme = Box::new(Filme {});
        assert_eq!(filme.reproduzir(), "Filme");
    }

    #[test]
    fn test_serie() {
        let serie = Box::new(Serie {});
        assert_eq!(serie.reproduzir(), "Serie");
    }
}
