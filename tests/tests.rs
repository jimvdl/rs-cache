use rscache::Cache;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rscache::LinkedListExt;

// TODO: change hasher (md5 sha1)

fn setup() -> Cache {
    Cache::new("A:\\GitHub\\rustscape\\data\\cache").expect("Error loading cache")
}

#[test]
fn initialize_cache() {
    let cache = setup();

    assert_eq!(22, cache.index_count());
}

#[test]
fn read_from_ref_tablek() {
    let cache = setup();

    let archive = cache.read(255, 10).unwrap().to_vec();

    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(&archive, &mut hasher);

    assert_eq!(13043865071375332525, hasher.finish());
    assert_eq!(77, archive.len());
}

#[test]
fn read_from_index_directly() {
    let cache = setup();

    let archive = cache.read(2, 10).unwrap().to_vec();

    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(&archive, &mut hasher);

    assert_eq!(1640288486933143591, hasher.finish());
    assert_eq!(260537, archive.len());
}

#[test]
fn encode_checksum() {
    let cache = setup();

    let checksum = cache.create_checksum().unwrap();
    let buffer = checksum.encode().unwrap();

    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(&buffer, &mut hasher);

    assert_eq!(10664786762202039525, hasher.finish());
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

#[test]
fn get_huffman_table() {
    let cache = setup();

    let huffman_table = cache.huffman_table();

    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(&huffman_table, &mut hasher);

    assert_eq!(10664786762202039525, hasher.finish());
    assert_eq!(173, huffman_table.len());
}