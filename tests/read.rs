#[cfg(test)]
mod test_util;

mod osrs {
    use super::test_util;

    #[test]
    fn metadata() {
        let cache = test_util::osrs_cache();
        let buffer = cache.read(255, 10).unwrap();
        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "64fb9fcf381a547bb7beafbc3b7ba4fd847f21ef");
        assert_eq!(buffer.len(), 77);
    }

    #[test]
    fn random_read() {
        let cache = test_util::osrs_cache();
        let buffer = cache.read(0, 191).unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "cd459f6ccfbd81c1e3bfadf899624f2519e207a9");
        assert_eq!(buffer.len(), 2055);
    }

    #[test]
    fn large_read() {
        let cache = test_util::osrs_cache();
        let buffer = cache.read(2, 10).unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "c6ee1518e9a39a42ecaf946c6c84a942cb3102f4");
        assert_eq!(buffer.len(), 260_537);
    }

    #[test]
    fn deep_archive() {
        let cache = test_util::osrs_cache();
        let buffer = cache.read(7, 24918).unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "fe91e9e9170a5a05ed2684c1db1169aa7ef4906e");
        assert_eq!(buffer.len(), 803);
    }

    #[test]
    fn single_data_len() {
        let cache = test_util::osrs_cache();
        let buffer = cache.read(3, 278).unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "036abb64d3f1734d892f69b1253a87639b7bcb44");
        assert_eq!(buffer.len(), 512);
    }

    #[test]
    fn double_data_len() {
        let cache = test_util::osrs_cache();
        let buffer = cache.read(0, 1077).unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "fbe9d365cf0c3efa94e0d4a2c5e607b28a1279b9");
        assert_eq!(buffer.len(), 1024);
    }

    #[test]
    fn fails() {
        let cache = test_util::osrs_cache();
        assert!(cache.read(2, 25_000).is_err());
    }
}

#[cfg(all(test, feature = "rs3"))]
mod rs3 {
    use super::test_util;

    #[test]
    fn random_0_read() {
        let cache = test_util::rs3_cache();
        let buffer = cache.read(0, 25).unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "81e455fc58fe5ac98fee4df5b78600bbf43e83f7");
        assert_eq!(buffer.len(), 1576);
    }

    #[test]
    fn between_single_double() {
        let cache = test_util::rs3_cache();
        let buffer = cache.read(7, 0).unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "b33919c6e4677abc6ec1c0bdd9557f820a163559");
        assert_eq!(buffer.len(), 529);
    }

    #[test]
    fn fails() {
        let cache = test_util::rs3_cache();
        assert!(cache.read(2, 25_000).is_err());
    }
}
