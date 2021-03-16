mod common;

#[cfg(feature = "osrs")]
mod osrs {
    use super::common;
    use rscache::OsrsCache;
    use rscache::util::osrs::Huffman;

    #[test]
    fn setup_cache() -> rscache::Result<()> {
        let cache = common::osrs::setup();

        assert!(cache.is_ok());
        assert_eq!(22, cache?.index_count());

        Ok(())
    }

    #[test]
    fn setup_cache_fails() -> rscache::Result<()> {
        let result = OsrsCache::new("./wrong/path");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn create_checksum() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        assert!(cache.create_checksum().is_ok());

        Ok(())
    }

    #[test]
    fn encode_checksum() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let checksum = cache.create_checksum()?;
        let buffer = checksum.encode_osrs()?;

        let hash = common::hash(&buffer);
        assert_eq!("0cb64350dc138e91bb83bc9c84b454631711f5de", &hash);
        assert_eq!(173, buffer.len());

        Ok(())
    }

    #[test]
    fn validate_checksum() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        
        let checksum = cache.create_checksum()?;

        let crcs = vec![1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367, 
                        4262755489, 2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569, 
                        153718440, 3849392898, 3628627685, 2813112885, 1461700456, 2751169400, 2927815226];
        let valid = checksum.validate(&crcs);

        assert!(valid);

        Ok(())
    }

    #[test]
    fn get_huffman_table() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let huffman_table = cache.huffman_table()?;

        let hash = common::hash(&huffman_table);
        assert_eq!("664e89cf25a0af7da138dd0f3904ca79cd1fe767", &hash);
        assert_eq!(256, huffman_table.len());

        Ok(())
    }

    #[test]
    fn huffman_decompress() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let huffman_table = cache.huffman_table()?;
        let huffman = Huffman::new(&huffman_table);

	    let compressed_msg = &[174, 128, 35, 32, 208, 96];
	    let decompressed_len = 8;
    
        let decompressed_msg = huffman.decompress(compressed_msg, decompressed_len);
	 
        let msg = String::from_utf8(decompressed_msg).unwrap_or_default();
        assert_eq!(msg, "rs-cache");

        Ok(())
    }
}

mod rs3 {
    use super::common;
    use rscache::Rs3Cache;

    #[test]
    fn setup_cache() -> rscache::Result<()> {
        let cache = common::rs3::setup();

        assert!(cache.is_ok());
        assert_eq!(58, cache?.index_count());

        Ok(())
    }

    #[test]
    fn setup_cache_fails() -> rscache::Result<()> {
        let result = Rs3Cache::new("./wrong/path");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn create_checksum() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;

        assert!(cache.create_checksum().is_ok());

        Ok(())
    }

    #[test]
    fn encode_checksum() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;

        let checksum = cache.create_checksum()?;
        let buffer = checksum.encode_rs3(common::rs3::EXPONENT, common::rs3::MODULUS)?;

        let hash = common::hash(&buffer);
        assert_eq!("118e0146af6cf288630357eec6298c34a2430065", &hash);
        assert_eq!(4681, buffer.len());

        Ok(())
    }
}