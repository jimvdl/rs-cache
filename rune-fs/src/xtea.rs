const ROUNDS: u32 = 32;
const RATIO: u32 = 0x9E3779B9;

/// Enciphers the data in-place with the given XTEA keys. Defaults to 32 rounds.
pub fn encipher(data: &mut [u8], keys: &[u32; 4]) {
    let blocks = data.len() / 8;

    let mut index = 0;
    for _ in 0..blocks {
        let mut v0 = u32::from_be_bytes([
            data[index],
            data[index + 1],
            data[index + 2],
            data[index + 3],
        ]);
        let mut v1 = u32::from_be_bytes([
            data[index + 4],
            data[index + 5],
            data[index + 6],
            data[index + 7],
        ]);
        let mut sum = 0_u32;
        for _ in 0..ROUNDS {
            v0 = v0.wrapping_sub(
                (((v1 << 4) ^ (v1 >> 5)).wrapping_add(v1))
                    ^ (sum.wrapping_add(keys[(sum & 3) as usize])),
            );
            sum = sum.wrapping_sub(RATIO);
            v1 = v1.wrapping_sub(
                (((v0 << 4) ^ (v0 >> 5)).wrapping_add(v0))
                    ^ (sum.wrapping_add(keys[((sum >> 11) & 3) as usize])),
            );
        }
        data[index..index + 4].copy_from_slice(&v0.to_be_bytes());
        data[index + 4..index + 8].copy_from_slice(&v1.to_be_bytes());

        index += 8;
    }
}

/// Deciphers the data in-place with the given XTEA keys. Defaults to 32 rounds.
pub fn decipher(data: &mut [u8], keys: &[u32; 4]) {
    let blocks = data.len() / 8;

    let mut index = 0;
    for _ in 0..blocks {
        let mut v0 = u32::from_be_bytes([
            data[index],
            data[index + 1],
            data[index + 2],
            data[index + 3],
        ]);
        let mut v1 = u32::from_be_bytes([
            data[index + 4],
            data[index + 5],
            data[index + 6],
            data[index + 7],
        ]);
        let mut sum = ROUNDS.wrapping_mul(RATIO);
        for _ in 0..ROUNDS {
            v1 = v1.wrapping_sub(
                (((v0 << 4) ^ (v0 >> 5)).wrapping_add(v0))
                    ^ (sum.wrapping_add(keys[((sum >> 11) & 3) as usize])),
            );
            sum = sum.wrapping_sub(RATIO);
            v0 = v0.wrapping_sub(
                (((v1 << 4) ^ (v1 >> 5)).wrapping_add(v1))
                    ^ (sum.wrapping_add(keys[(sum & 3) as usize])),
            );
        }
        data[index..index + 4].copy_from_slice(&v0.to_be_bytes());
        data[index + 4..index + 8].copy_from_slice(&v1.to_be_bytes());

        index += 8;
    }
}
