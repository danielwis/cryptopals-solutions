pub fn xor_slices(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.into_iter().zip(b).map(|(a, b)| a ^ b).collect()
}
