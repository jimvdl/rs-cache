use rscache::{ Cache, LinkedListExt };

fn setup() -> Cache {
    Cache::new("./tests/cache").expect("Error loading cache")
}

#[test]
fn initialize_cache() {
    let cache = setup();

    assert_eq!(22, cache.index_count());
}

#[test]
fn read_from_ref_table() {
    let cache = setup();

    let archive = cache.read(255, 10).unwrap().to_vec();

    let mut m = sha1::Sha1::new();
    m.update(&archive);

    assert_eq!("64fb9fcf381a547bb7beafbc3b7ba4fd847f21ef", &m.digest().to_string());
    assert_eq!(77, archive.len());
}

#[test]
fn read_from_index_directly() {
    let cache = setup();

    let archive = cache.read(2, 10).unwrap().to_vec();

    let mut m = sha1::Sha1::new();
    m.update(&archive);

    assert_eq!("c6ee1518e9a39a42ecaf946c6c84a942cb3102f4", &m.digest().to_string());
    assert_eq!(260537, archive.len());
}

#[test]
fn encode_checksum() {
    let cache = setup();

    let checksum = cache.create_checksum().unwrap();
    let buffer = checksum.encode().unwrap();

    let mut m = sha1::Sha1::new();
    m.update(&buffer);

    assert_eq!("1a7cd53f7766970d5f8d7aa9c3fc7a0984d1d7d5", &m.digest().to_string());
    assert_eq!(173, buffer.len());
}

#[test]
fn validate_checksum() {
    let cache = setup();

    let crcs = vec![1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367, 
                    4262755489, 2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569, 
                    153718440, 3849392898, 0, 2813112885, 1461700456, 2751169400, 2927815226];
    let valid = cache.create_checksum().unwrap().validate_crcs(&crcs);

    assert_eq!(true, valid);
}

// #[test]
// fn get_huffman_table() {
//     let cache = setup();

//     let huffman_table = cache.huffman_table();

//     let mut m = sha1::Sha1::new();
//     m.update(&huffman_table);

//     assert_eq!("10664786762202039525", &m.digest().to_string());
//     assert_eq!(173, huffman_table.len());
// }