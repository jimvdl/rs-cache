use rscache::CacheError;

mod common;

#[test]
fn initialize_cache() -> Result<(), CacheError> {
    let cache = common::setup()?;

    assert_eq!(22, cache.index_count());
    Ok(())
}

#[test]
fn encode_checksum() -> Result<(), CacheError> {
    let cache = common::setup()?;

    let checksum = cache.create_checksum()?;
    let buffer = checksum.encode()?;

    let hash = common::hash(&buffer);

    assert_eq!("1a7cd53f7766970d5f8d7aa9c3fc7a0984d1d7d5", &hash);
    assert_eq!(173, buffer.len());
    Ok(())
}

#[test]
fn validate_checksum() -> Result<(), CacheError> {
    let cache = common::setup()?;
    
    let checksum = cache.create_checksum()?;

    let crcs = vec![1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367, 
                    4262755489, 2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569, 
                    153718440, 3849392898, 0, 2813112885, 1461700456, 2751169400, 2927815226];
    let valid = checksum.validate_crcs(&crcs);

    assert_eq!(true, valid);
    Ok(())
}

#[test]
fn get_huffman_table() -> Result<(), CacheError> {
    let cache = common::setup()?;

    let huffman_table = cache.huffman_table()?.to_vec();

    let hash = common::hash(&huffman_table);

    assert_eq!("664e89cf25a0af7da138dd0f3904ca79cd1fe767", &hash);
    assert_eq!(256, huffman_table.len());
    Ok(())
}