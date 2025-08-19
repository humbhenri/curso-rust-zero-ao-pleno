fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut maior = list[0];
    for t in list {
        if *t > maior {
            maior = *t;
        }
    }
    maior
}

fn main() {
    let vs = [1, 2, 3];
    let maior = largest(&vs);
    println!("{}", maior);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number() {
        let ns = [1, -1, 100];
        assert_eq!(largest(&ns), 100);
    }

    #[test]
    fn test_largest_character() {
        let chars = ['a', ' ', 'b'];
        assert_eq!(largest(&chars), 'b');
    }
}
