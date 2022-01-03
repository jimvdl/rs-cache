const ROUNDS: u32 = 32;
const RATIO: u32 = 0x9E3779B9;

/// Enciphers the data with the given XTEA keys. Defaults to 32 rounds.
#[inline]
pub fn encipher(data: &[u8], keys: &[u32; 4]) -> Vec<u8> {
    let blocks = data.len() / 8;
    let mut buf = data.to_vec();

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
        buf[index..index + 4].copy_from_slice(&v0.to_be_bytes());
        buf[index + 4..index + 8].copy_from_slice(&v1.to_be_bytes());

        index += 8;
    }

    buf
}

/// Deciphers the data with the given XTEA keys. Defaults to 32 rounds.
#[inline]
pub fn decipher(data: &[u8], keys: &[u32; 4]) -> Vec<u8> {
    let blocks = data.len() / 8;
    let mut buf = data.to_vec();

    let mut index = 0;
    for _ in 0..blocks {
        let mut v0 =
            u32::from_be_bytes([buf[index], buf[index + 1], buf[index + 2], buf[index + 3]]);
        let mut v1 = u32::from_be_bytes([
            buf[index + 4],
            buf[index + 5],
            buf[index + 6],
            buf[index + 7],
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
        buf[index..index + 4].copy_from_slice(&v0.to_be_bytes());
        buf[index + 4..index + 8].copy_from_slice(&v1.to_be_bytes());

        index += 8;
    }

    buf
}
