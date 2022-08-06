pub fn bytes_to_vec(b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(b.len());

    out.extend_from_slice(b);
    out
}
