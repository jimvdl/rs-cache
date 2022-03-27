#[cfg(test)]
mod test_util;

#[cfg(test)]
mod osrs {
    use super::test_util;
    use rscache::Cache;

    #[test]
    fn new() {
        assert!(Cache::new("./data/osrs_cache").is_ok());
    }

    #[test]
    fn new_wrong_path() {
        assert!(Cache::new("./wrong/path").is_err());
    }

    #[test]
    fn huffman_table() {
        let cache = test_util::osrs_cache();
        let buffer = cache.huffman_table().unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "664e89cf25a0af7da138dd0f3904ca79cd1fe767");
        assert_eq!(buffer.len(), 256);
    }
}

#[cfg(all(test, feature = "rs3"))]
mod rs3 {
    use super::test_util;
    use rscache::Cache;

    #[test]
    fn new() {
        assert!(Cache::new("./data/rs3_cache").is_ok());
    }

    #[test]
    fn new_wrong_path() {
        assert!(Cache::new("./wrong/path").is_err());
    }

    #[test]
    fn huffman_table() {
        let cache = test_util::rs3_cache();
        let buffer = cache.huffman_table().unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "664e89cf25a0af7da138dd0f3904ca79cd1fe767");
        assert_eq!(buffer.len(), 256);
    }
}