# RS-Cache
[![Build](https://github.com/jimvdl/rs-cache/workflows/build/badge.svg)](https://github.com/jimvdl/rs-cache)
[![Crate](https://img.shields.io/crates/v/rs-cache)](https://crates.io/crates/rs-cache)
[![OSRS Version](https://img.shields.io/badge/OSRS-180-blue)]()
[![RS3 Version](https://img.shields.io/badge/RS3-904-blue)]()
[![API](https://docs.rs/rs-cache/badge.svg)](https://docs.rs/rs-cache)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.41%2B-yellow)](https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html)
[![License](https://img.shields.io/crates/l/rs-cache?color=black)](https://github.com/jimvdl/rs-cache/blob/master/LICENSE)

A simple-to-use basic RuneScape cache utility. RS-Cache provides utilities to interact with the RuneScape cache. 

Useful links:\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/5/5d/Fire_rune_detail.png/800px-Fire_rune_detail.png?07ed5" width="10"> &nbsp;[Releases](https://github.com/jimvdl/rs-cache/releases)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/7/74/Water_rune_detail.png/800px-Water_rune_detail.png?4e790" width="10"> &nbsp;[Documentation](https://docs.rs/rs-cache)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/e/ef/Nature_rune_detail.png/800px-Nature_rune_detail.png?a062f" width="10"> &nbsp;[Examples](examples/)

## Quick Start

The quickest and easiest way to get started is by using the 
[`Cache`]([type.Cache.html](https://docs.rs/rs-cache/0.7/rscache/type.Cache.html)) struct.

```rust
use rscache::Cache;

fn main() -> rscache::Result<()> {
    let cache = Cache::new("./data/osrs_cache")?;

    let index_id = 2; // Config index.
    let archive_id = 10; // Archive containing item definitions.

    let buffer: Vec<u8> = cache.read(index_id, archive_id)?;

    Ok(())
}
```

The public API of this crate is still evolving.
Currently supports both OSRS & RS3, although both are still limited.

The [osrs specifications](osrs_specifications.md) and [rs3 specifications](rs3_specifications.md) documents contain a detailed description of the design of the corresponding cache for educational purposes. Both documents are still a work in progress and are possibly incomplete.

Integration tests are running on Oldschool RuneScape version 180, which you can run at any time because the cache is included in the `./data/osrs_cache` directory. RS3 Integration tests are running on version 904. The RS3 cache is too large to include on GitHub.

The minimum supported `rustc` version is `1.41`.

This crate is passively maintained. Additional features will be implemented once they are needed for my own server.
The public API of this crate is still evolving due to constant discoveries and overhauls to improve the overal usage.
__If you require a certain feature feel free to open an issue.__

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rs-cache = "0.7"
```

Examples can be found in the [examples](examples/) directory. These examples include setting up your own custom cache and how the update protocol could be handled.

## Acknowledgements

The following sources aided with the development of this crate:\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/d/dc/Cosmic_rune_detail.png/800px-Cosmic_rune_detail.png?734d1" width="10"> &nbsp;[OpenRS](https://www.rune-server.ee/runescape-development/rs-503-client-server/downloads/312510-openrs-cache-library.html)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/f/f3/Air_rune_detail.png/800px-Air_rune_detail.png?b7f49" width="10"> &nbsp;[RuneLite](https://runelite.net/)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/0/0f/Law_rune_detail.png/800px-Law_rune_detail.png?dc1f3" width="10"> &nbsp;[OSRS Cache Parsing Blog](https://www.osrsbox.com/blog/2018/07/26/osrs-cache-research-extract-cache-definitions/)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/a/ae/Chaos_rune_detail.png/800px-Chaos_rune_detail.png?0d8cb" width="10"> &nbsp;[RSMod](https://github.com/Tomm0017/rsmod)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/8/8b/Soul_rune_detail.png/800px-Soul_rune_detail.png?75ada" width="10"> &nbsp;[Librsfs](https://github.com/Velocity-/librsfs)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/c/c1/Blood_rune_detail.png/800px-Blood_rune_detail.png?2cf9e" width="10"> &nbsp;[OSRSBox](https://www.osrsbox.com/)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/7/72/Earth_rune_detail.png/800px-Earth_rune_detail.png?991bd" width="10"> &nbsp;[Jagex-Store-5](https://github.com/guthix/Jagex-Store-5)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/7/70/Wrath_rune.png?3a737" width="10"> &nbsp;[Matrix 876](https://www.rune-server.ee/runescape-development/rs-503-client-server/downloads/648085-matrix-3-876-recommended-876-rs3-server.html)


## License
RS-Cache is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.