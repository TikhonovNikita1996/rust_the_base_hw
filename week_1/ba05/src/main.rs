pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    for (row, line) in lines.iter().enumerate() {
        let mut byte = 0u8;
        for (i, ch) in line.chars().enumerate() {
            if ch == '#' {
                byte |= 1 << (7 - i);
            }
        }
        bytes[row] = byte;
    }
    bytes
}

pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
    std::array::from_fn(|row| {
        (0..8)
            .map(|i| if bytes[row] & (1 << (7 - i)) != 0 { '#' } else { '.' })
            .collect()
    })
}

pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
    std::array::from_fn(|i| !bytes[i])
}

fn main() {
    let image = [
        "..####..",
        ".#....#.",
        "#.#..#.#",
        "#..##..#",
        "#......#",
        "#.#..#.#",
        ".#....#.",
        "..####..",
    ];

    let bytes = parse_bitmap_8x8(image);

    println!("Bytes:");
    for byte in bytes {
        println!("{:08b}  0x{:02X}", byte, byte);
    }

    println!();
    println!("Rendered:");
    for line in render_bitmap_8x8(bytes) {
        println!("{}", line);
    }

    println!();
    println!("Inverted:");
    for line in render_bitmap_8x8(invert_bitmap_8x8(bytes)) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitmap() {
        let image = [
            "..####..",
            ".#....#.",
            "#.#..#.#",
            "#..##..#",
            "#......#",
            "#.#..#.#",
            ".#....#.",
            "..####..",
        ];
        let bytes = parse_bitmap_8x8(image);
        assert_eq!(bytes[0], 0b0011_1100);
        assert_eq!(bytes[1], 0b0100_0010);
        assert_eq!(bytes[2], 0b1010_0101);
        assert_eq!(bytes[3], 0b1001_1001);
        assert_eq!(bytes[4], 0b1000_0001);
        assert_eq!(bytes[5], 0b1010_0101);
        assert_eq!(bytes[6], 0b0100_0010);
        assert_eq!(bytes[7], 0b0011_1100);
    }

    #[test]
    fn test_render_bitmap() {
        let bytes = [
            0b0011_1100u8,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b0011_1100,
        ];
        let rendered = render_bitmap_8x8(bytes);
        assert_eq!(rendered[0], "..####..");
        assert_eq!(rendered[1], ".#....#.");
        assert_eq!(rendered[7], "..####..");
    }

    #[test]
    fn test_invert_bitmap() {
        let image = [
            "..####..",
            ".#....#.",
            "#.#..#.#",
            "#..##..#",
            "#......#",
            "#.#..#.#",
            ".#....#.",
            "..####..",
        ];
        let bytes = parse_bitmap_8x8(image);
        let inverted = render_bitmap_8x8(invert_bitmap_8x8(bytes));
        assert_eq!(inverted[0], "##....##");
        assert_eq!(inverted[1], "#.####.#");
        assert_eq!(inverted[4], ".######.");
        assert_eq!(inverted[7], "##....##");
    }

    #[test]
    fn test_roundtrip() {
        let image = [
            "..####..",
            ".#....#.",
            "#.#..#.#",
            "#..##..#",
            "#......#",
            "#.#..#.#",
            ".#....#.",
            "..####..",
        ];
        let bytes = parse_bitmap_8x8(image);
        let rendered = render_bitmap_8x8(bytes);
        for (i, line) in image.iter().enumerate() {
            assert_eq!(rendered[i], *line);
        }
    }
}
