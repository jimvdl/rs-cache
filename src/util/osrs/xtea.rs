const ROUNDS: u32 = 32;

#[inline]
pub fn decipher(keys: &[u32], data: &[u8], start: usize, end: usize) -> Vec<u8> {
    let blocks = (end - start) / 8;
    let mut buf = data.to_vec();

    let mut index = 0;
    for _ in 0..blocks {
        let mut v0 = u32::from_be_bytes([buf[index], buf[index + 1], buf[index + 2], buf[index + 3]]);
        let mut v1 = u32::from_be_bytes([buf[index + 4], buf[index + 5], buf[index + 6], buf[index + 7]]);
        let delta = 0x9E3779B9;
        let mut sum = ROUNDS.wrapping_mul(delta);
        for _ in 0..ROUNDS {
            v1 = v1.wrapping_sub((((v0 << 4) ^ (v0 >> 5)).wrapping_add(v0)) ^ (sum.wrapping_add(keys[((sum >> 11) & 3) as usize])));
            sum = sum.wrapping_sub(delta);
            v0 = v0.wrapping_sub((((v1 << 4) ^ (v1 >> 5)).wrapping_add(v1)) ^ (sum.wrapping_add(keys[(sum & 3) as usize])));
        }
        buf[index..index + 4].copy_from_slice(&v0.to_be_bytes());
        buf[index + 4..index + 8].copy_from_slice(&v1.to_be_bytes());

        index += 8;
    }

    buf
}
