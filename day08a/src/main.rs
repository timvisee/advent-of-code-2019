fn main() {
    println!(
        "Checksum: {}",
        std::fs::read("./input.txt")
            .unwrap()
            .chunks_exact(25 * 6)
            .min_by_key(|c| bytecount::count(c, b'0'))
            .map(|c| bytecount::count(c, b'1') * bytecount::count(c, b'2'))
            .unwrap()
    );
}
