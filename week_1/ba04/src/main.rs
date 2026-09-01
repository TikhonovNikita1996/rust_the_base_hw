pub fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}

pub fn add_u8_wrapping(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

pub fn add_u8_saturating(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}

fn main() {
    println!("add_u8_checked(10, 20) = {:?}", add_u8_checked(10, 20));
    println!("add_u8_checked(255, 1) = {:?}", add_u8_checked(255, 1));
    println!("add_u8_wrapping(10, 20) = {}", add_u8_wrapping(10, 20));
    println!("add_u8_wrapping(255, 1) = {}", add_u8_wrapping(255, 1));
    println!("add_u8_saturating(10, 20) = {}", add_u8_saturating(10, 20));
    println!("add_u8_saturating(255, 1) = {}", add_u8_saturating(255, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_overflow_modes() {
        assert_eq!(add_u8_checked(255, 1), None);
        assert_eq!(add_u8_wrapping(255, 1), 0);
        assert_eq!(add_u8_saturating(255, 1), 255);

        assert_eq!(add_u8_checked(10, 20), Some(30));
        assert_eq!(add_u8_wrapping(10, 20), 30);
        assert_eq!(add_u8_saturating(10, 20), 30);
    }
}
