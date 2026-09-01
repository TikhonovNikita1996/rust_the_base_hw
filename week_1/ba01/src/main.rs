use std::io::{self, Read};

fn count_bytes(buf: &[u8]) -> usize {
    buf.len()
}

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();
    println!("{}", count_bytes(&buf));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(count_bytes(b""), 0);
    }

    #[test]
    fn test_foo_with_newline() {
        assert_eq!(count_bytes(b"foo\n"), 4);
    }

    #[test]
    fn test_no_newline() {
        assert_eq!(count_bytes(b"hello"), 5);
    }

    #[test]
    fn test_emoji_crab() {
        assert_eq!(count_bytes("🦀\n".as_bytes()), 5);
    }

    #[test]
    fn test_binary_data() {
        assert_eq!(count_bytes(&[0x00, 0xFF, 0x42]), 3);
    }

    #[test]
    fn test_1000_bytes() {
        let buf: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        assert_eq!(count_bytes(&buf), 1000);
    }
}
