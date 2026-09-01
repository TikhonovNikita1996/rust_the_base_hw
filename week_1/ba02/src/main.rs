use std::io::{self, Read};

fn count(buf: &[u8]) -> (usize, usize, usize) {
    let bytes = buf.len();
    let mut lines = 0usize;
    let mut words = 0usize;
    let mut in_word = false;

    for &b in buf {
        if b == b'\n' {
            lines += 1;
        }
        if b.is_ascii_whitespace() {
            in_word = false;
        } else {
            if !in_word {
                words += 1;
            }
            in_word = true;
        }
    }

    (lines, words, bytes)
}

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();
    let (lines, words, bytes) = count(&buf);
    println!("{} {} {}", lines, words, bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(count(b""), (0, 0, 0));
    }

    #[test]
    fn test_foo_newline() {
        assert_eq!(count(b"foo\n"), (1, 1, 4));
    }

    #[test]
    fn test_no_newline() {
        assert_eq!(count(b"hello"), (0, 1, 5));
    }

    #[test]
    fn test_with_newline() {
        assert_eq!(count(b"hello\n"), (1, 1, 6));
    }

    #[test]
    fn test_two_words() {
        assert_eq!(count(b"hello rust\n"), (1, 2, 11));
    }

    #[test]
    fn test_leading_trailing_spaces() {
        assert_eq!(count(b" hi \n"), (1, 1, 5));
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(count(b"hello      rust\n"), (1, 2, 16));
    }

    #[test]
    fn test_tab_and_newline() {
        assert_eq!(count(b"a\tb\nc"), (1, 3, 5));
    }

    #[test]
    fn test_yes_1000() {
        let buf: Vec<u8> = b"y\n".repeat(500);
        assert_eq!(count(&buf), (500, 500, 1000));
    }
}
