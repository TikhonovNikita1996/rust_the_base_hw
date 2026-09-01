fn insertion_sort(args: &mut Vec<String>) {
    for i in 1..args.len() {
        let mut j = i;
        while j > 0 && args[j - 1] > args[j] {
            args.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    insertion_sort(&mut args);
    for arg in &args {
        println!("{}", arg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_sorted() {
        let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        insertion_sort(&mut v);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_reverse_order() {
        let mut v = vec!["e".to_string(), "d".to_string(), "c".to_string(), "b".to_string(), "a".to_string()];
        insertion_sort(&mut v);
        assert_eq!(v, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_uppercase_before_lowercase() {
        let mut v = vec!["a".to_string(), "A".to_string()];
        insertion_sort(&mut v);
        assert_eq!(v, vec!["A", "a"]);
    }

    #[test]
    fn test_duplicates() {
        let mut v = vec!["b".to_string(), "a".to_string(), "a".to_string()];
        insertion_sort(&mut v);
        assert_eq!(v, vec!["a", "a", "b"]);
    }

    #[test]
    fn test_empty() {
        let mut v: Vec<String> = vec![];
        insertion_sort(&mut v);
        assert_eq!(v, Vec::<String>::new());
    }

    #[test]
    fn test_with_punctuation() {
        let mut v = vec!["world,".to_string(), "hello,".to_string(), "a".to_string()];
        insertion_sort(&mut v);
        assert_eq!(v, vec!["a", "hello,", "world,"]);
    }
}
