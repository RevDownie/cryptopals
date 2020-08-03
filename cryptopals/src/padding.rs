/// Returns a copy of the data padded to be aligned to the given size
/// The length of padding is the data used to actually pad
///
pub fn padded(data: &[u8], alignment: usize) -> Vec<u8> {
    let padding = (alignment - (data.len() % alignment)) % alignment;
    let padded_len = data.len() + padding;
    let mut padded = Vec::with_capacity(padded_len);
    padded.extend(data);
    padded.extend((0..padding).map(|_| padding as u8));
    padded
}

/// Unit tests
///
#[test]
fn test_padding_no_padding() {
    let p = padded(b"01234567", 8);
    assert!(p.len() == 8);
}

#[test]
fn test_padding_smaller_alignment() {
    let p = padded(b"01234567", 5);
    assert!(p.len() == 10);
    assert!(p[9] == 2);
}

#[test]
fn test_padding_larger_alignment() {
    let p = padded(b"01234567", 10);
    assert!(p.len() == 10);
    assert!(p[9] == 2);
}
