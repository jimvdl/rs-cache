mod test_util;

use rscache::checksum::Checksum;

#[test]
fn new() {
    let cache = test_util::osrs_cache();

    assert!(Checksum::new(&cache).is_ok());
    assert!(cache.checksum().is_ok());
}

#[test]
fn encode() {
    let cache = test_util::osrs_cache();
    let buffer = Checksum::new(&cache).unwrap().encode().unwrap();

    let hash = test_util::hash(&buffer);
    assert_eq!(&hash, "0cb64350dc138e91bb83bc9c84b454631711f5de");
    assert_eq!(buffer.len(), 173);
}

#[test]
fn validate() {
    let cache = test_util::osrs_cache();
    let checksum = Checksum::new(&cache).unwrap();

    let crcs = [
        1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367, 4262755489,
        2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569, 153718440,
        3849392898, 3628627685, 2813112885, 1461700456, 2751169400, 2927815226,
    ];

    assert!(checksum.validate(crcs).is_ok());
}

#[test]
fn validate_as_ref() {
    let cache = test_util::osrs_cache();
    let checksum = Checksum::new(&cache).unwrap();

    let crcs = [
        &1593884597, &1029608590, &16840364, &4209099954, &3716821437, &165713182, &686540367, &4262755489,
        &2208636505, &3047082366, &586413816, &2890424900, &3411535427, &3178880569, &153718440,
        &3849392898, &3628627685, &2813112885, &1461700456, &2751169400, &2927815226,
    ];

    assert!(checksum.validate(crcs).is_ok());
}

#[test]
fn invalid_crc() {
    use rscache::error::ValidateError;

    let cache = test_util::osrs_cache();
    let checksum = Checksum::new(&cache).unwrap();

    let crcs = [
        1593884597, 1029608590, 16840364, 4209098954, 3716821437, 165713182, 686540367, 4262755489,
        2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569, 153718440,
        3849392898, 3628627685, 2813112885, 1461700456, 2751169400, 2927815226,
    ];

    assert_eq!(
        checksum.validate(crcs),
        Err(ValidateError::InvalidCrc {
            idx: 3,
            external: 4209098954,
            internal: 4209099954
        })
    );
}

#[test]
fn invalid_len() {
    use rscache::error::ValidateError;

    let cache = test_util::osrs_cache();
    let checksum = Checksum::new(&cache).unwrap();

    let crcs = [
        1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367, 4262755489,
        2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569, 153718440,
        3849392898, 3628627685, 2813112885, 1461700456, 2751169400,
    ];

    assert_eq!(
        checksum.validate(crcs),
        Err(ValidateError::InvalidLength {
            expected: 21,
            actual: 20
        })
    );
}

#[cfg(all(test, feature = "rs3"))]
mod rsa {
    use rscache::checksum::{RsaChecksum, RsaKeys};
    use super::test_util;
    pub const EXPONENT: &[u8] = b"5206580307236375668350588432916871591810765290737810323990754121164270399789630501436083337726278206128394461017374810549461689174118305784406140446740993";
    pub const MODULUS: &[u8] = b"6950273013450460376345707589939362735767433035117300645755821424559380572176824658371246045200577956729474374073582306250298535718024104420271215590565201";

    #[test]
    fn with_keys() {
        let cache = test_util::rs3_cache();
        let keys = RsaKeys::new(EXPONENT, MODULUS);
        let buffer = RsaChecksum::with_keys(&cache, keys).unwrap().encode().unwrap();

        let hash = test_util::hash(&buffer);
        assert_eq!(&hash, "118e0146af6cf288630357eec6298c34a2430065");
        assert_eq!(buffer.len(), 4681);
    }
}