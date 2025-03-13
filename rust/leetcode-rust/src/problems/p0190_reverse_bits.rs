pub fn reverse_bits(x: u32) -> u32 {
    let mut y: u32 = 0;
    (0..32).for_each(|i| {
        y |= (x >> i & 1) << (31 - i);
    });
    y
}