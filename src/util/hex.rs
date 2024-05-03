pub(crate) fn decode(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks_exact(2)
        .map(|pair| val(pair[0]) << 4 | val(pair[1]))
        .collect()
}

fn val(c: u8) -> u8 {
    match c {
        b'A'..=b'F' => c - b'A' + 10,
        b'a'..=b'f' => c - b'a' + 10,
        b'0'..=b'9' => c - b'0',
        _ => panic!("shiit"),
    }
}

#[cfg(test)]
mod test {
    use super::decode;

    #[test]
    fn test_decode() {
        let hex = "48656c6c6f20576f726c64";
        let s = String::from("Hello World");

        assert_eq!(s, String::from_utf8(decode(hex).to_vec()).unwrap());
    }
}
