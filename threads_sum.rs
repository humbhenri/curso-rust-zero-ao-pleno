use std::ops::Add;
use std::thread;

pub fn calculate_sum<T>(numbers: &[T], num_threads: usize) -> T
where
    T: Add<Output = T> + Send + Copy + Default + 'static,
{
    if numbers.is_empty() {
        return T::default();
    }

    let chunk_size = (numbers.len() + num_threads - 1) / num_threads;
    let mut handles = Vec::new();

    for chunk in numbers.chunks(chunk_size) {
        let part: Vec<T> = chunk.to_vec();
        let handle = thread::spawn(move || part.into_iter().fold(T::default(), |acc, x| acc + x));
        handles.push(handle);
    }

    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .fold(T::default(), |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum_i32() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = calculate_sum(&numbers, 3);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_calculate_sum_f64() {
        let numbers = vec![1.5, 2.5, 3.0, 4.0];
        let result = calculate_sum(&numbers, 2);
        assert!((result - 11.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_empty_slice() {
        let numbers: Vec<i32> = vec![];
        let result = calculate_sum(&numbers, 4);
        assert_eq!(result, 0);
    }
}
