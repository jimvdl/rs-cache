/// Decompresses chat messages.
/// 
/// # Examples
///
/// ```
/// # use rscache::Cache;
/// use rscache::util::Huffman;
///
/// # fn main() -> Result<(), rscache::Error> {
/// # let cache = Cache::new("./data/osrs_cache")?;
/// let huffman_tbl = cache.huffman_table()?;
/// let huffman = Huffman::new(&huffman_tbl);
///
/// let compressed_msg = &[174, 128, 35, 32, 208, 96];
/// let decompressed_len = 8; // client will include this in the chat packet.
///
/// let decompressed_msg = huffman.decompress(compressed_msg, decompressed_len);
///
/// if let Ok(msg) = String::from_utf8(decompressed_msg) {
///     assert_eq!(msg, "rs-cache");
/// }
/// # Ok(())
/// # }
/// ```

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Huffman {
    keys: Vec<i32>,
}

impl Huffman {
    /// Initializes the Huffman struct with the given sizes.
    ///
    /// The sizes can be found in the cache.
    /// Call the [`huffman_table()`](../struct.Cache.html#method.huffman_table) function to get the huffman table which
    /// contains the sizes needed to initialize this struct.
    pub fn new(sizes: &[u8]) -> Self {
        let i_2 = sizes.len();
        let mut masks: Vec<i32> = vec![0; i_2];
        let mut ints_3: Vec<i32> = vec![0; 33];
        let mut keys: Vec<i32> = vec![0; 8];
        let mut i_4 = 0;

        for i_5 in 0..i_2 {
            let b_6 = sizes[i_5];
            if b_6 != 0 {
                let i_7 = 1 << (32 - b_6);
                let i_8 = ints_3[b_6 as usize];
                masks[i_5] = i_8;
                let mut i_9 = 0;
                let mut i_10 = 0;
                let mut i_11 = 0;
                let mut i_12_1 = 0;
                if i_8 & i_7 != 0 {
                    i_9 = ints_3[(b_6 - 1) as usize];
                } else {
                    i_9 = i_8 | i_7;

                    i_10 = (b_6 - 1) as i32;
                    while i_10 >= 1 {
                        i_11 = ints_3[i_10 as usize];
                        if i_11 != i_8 {
                            break;
                        }

                        i_12_1 = 1 << (32 - i_10);
                        if i_11 & i_12_1 != 0 {
                            ints_3[i_10 as usize] = ints_3[(i_10 - 1) as usize];
                            break;
                        }

                        ints_3[i_10 as usize] = i_11 | i_12_1;
                        i_10 -= 1;
                    }
                }

                ints_3[b_6 as usize] = i_9;

                i_10 = (b_6 + 1) as i32;
                while i_10 <= 32 {
                    if ints_3[i_10 as usize] == i_8 {
                        ints_3[i_10 as usize] = i_9;
                    }
                    i_10 += 1;
                }

                i_10 = 0;

                i_11 = 0;
                while i_11 < b_6 as i32 {
                    i_12_1 = (i32::MIN as u32 >> i_11) as i32;
                    if i_8 & i_12_1 != 0 {
                        if keys[i_10 as usize] == 0 {
                            keys[i_10 as usize] = i_4;
                        }

                        i_10 = keys[i_10 as usize];
                    } else {
                        i_10 += 1;
                    }

                    if i_10 as usize >= keys.len() {
                        let mut ints_13_vec = vec![0; keys.len() * 2];

                        ints_13_vec[..keys.len()].clone_from_slice(&keys[..]);

                        keys = ints_13_vec;
                    }

                    i_11 += 1;
                }

                keys[i_10 as usize] = (!i_5) as i32;
                if i_10 >= i_4 {
                    i_4 = i_10 + 1;
                }
            }
        }

        Self { keys }
    }

    /// Decompresses the given buffer.
    ///
    /// The buffer is normally an encoded chat message which will be decoded into
    /// the original message. This helps limit chat packet sizes.
    ///
    /// # Panics
    ///
    /// Panics if the decompressed length == 0
    pub fn decompress(&self, compressed: &[u8], decompressed_len: usize) -> Vec<u8> {
        let mut decompressed = vec![0; decompressed_len];

        let i_2 = 0;
        let mut i_4 = 0;
        if decompressed_len == 0 {
            panic!("Huffman decompressed message length can't be 0.");
        }
        let mut i_7 = 0;
        let mut i_8 = i_2;

        loop {
            if i_4 >= decompressed_len {
                break;
            }

            let b_9 = compressed[i_8 as usize];
            if b_9 > 127 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            let mut i_10 = 0;
            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            if b_9 & 0x40 != 0 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            if b_9 & 0x20 != 0 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            if b_9 & 0x10 != 0 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            if b_9 & 0x8 != 0 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            if b_9 & 0x4 != 0 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            if b_9 & 0x2 != 0 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            if b_9 & 0x1 != 0 {
                i_7 = self.keys[i_7 as usize];
            } else {
                i_7 += 1;
            }

            i_10_keys(&mut i_10, &self.keys, &mut i_7, &mut i_4, &mut decompressed);

            i_8 += 1;
        }

        decompressed
    }
}

fn i_10_keys(
    i_10: &mut i32,
    keys: &[i32],
    i_7: &mut i32,
    i_4: &mut usize,
    decompressed: &mut [u8],
) {
    *i_10 = keys[*i_7 as usize];
    if *i_10 < 0 {
        decompressed[*i_4] = (!*i_10) as u8;
        *i_4 += 1;
        *i_7 = 0;
    }
}
