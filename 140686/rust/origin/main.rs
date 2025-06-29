fn main() {
    let dst: Vec<u8> = Vec::new();
    let len = broken_func(2, dst);
    assert_eq!(len, 8);
    eprintln!("{len}");
}

#[inline(never)]
#[no_mangle]
pub fn broken_func(version: usize, mut dst: Vec<u8>) -> usize {
    match version {
        1 => dst.extend_from_slice(b"aaaaaaaa"),
        2 => dst.extend_from_slice(b"bbbbbbbb"),
        3 => dst.extend_from_slice(b"bbbbbbbb"),
        _ => panic!(),
    }
    dst.len()
}
