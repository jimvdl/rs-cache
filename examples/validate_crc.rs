use rscache::Cache;

fn main() -> rscache::Result<()> {
    let cache = Cache::new("./data/cache")?;
    let checksum = cache.create_checksum()?;

    let crcs = vec![1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367, 
                    4262755489, 2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569, 
                    153718440, 3849392898, 0, 2813112885, 1461700456, 2751169400, 2927815226];
    let valid = checksum.validate_crcs(&crcs);

    assert_eq!(valid, true);

    Ok(())
}