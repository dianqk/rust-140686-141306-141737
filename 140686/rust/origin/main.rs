fn main() {
    let dst: Vec<u8> = Vec::new();
    let len = broken_func(2, dst);
    assert_eq!(len, 8);
    eprintln!("{len}");
}

pub fn extend_from_slice(dst: &mut Vec<u8>, other: &[u8]) {
    dst.extend_from_slice(other);
}

#[inline(never)]
#[no_mangle]
pub fn broken_func(version: usize, mut dst: Vec<u8>) -> usize {
    match version {
        1 => extend_from_slice(&mut dst, b"aaaaaaaa"),
        2 => extend_from_slice(&mut dst, b"bbbbbbbb"),
        3 => extend_from_slice(&mut dst, b"bbbbbbbb"),
        _ => panic!(),
    }
    dst.len()
}
