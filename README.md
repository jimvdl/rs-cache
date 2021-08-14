# RS-Cache
[![Build](https://github.com/jimvdl/rs-cache/workflows/build/badge.svg)](https://github.com/jimvdl/rs-cache)
[![Crate](https://img.shields.io/crates/v/rs-cache)](https://crates.io/crates/rs-cache)
[![OSRS Revision](https://img.shields.io/badge/OSRS-180-blue)]()
[![RS3 Revision](https://img.shields.io/badge/RS3-904-blue)]()
[![API](https://docs.rs/rs-cache/badge.svg)](https://docs.rs/rs-cache)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.41%2B-yellow)](https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html)
[![License](https://img.shields.io/crates/l/rs-cache?color=black)](https://github.com/jimvdl/rs-cache/blob/master/LICENSE)

A simple-to-use basic RuneScape cache utility. RS-Cache provides utilities to interact with the RuneScape cache. 

Follow this [Quick Start](https://docs.rs/rs-cache/#quick-start) guide to begin using rs-cache.

Useful links:\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/5/5d/Fire_rune_detail.png/800px-Fire_rune_detail.png?07ed5" width="10"> &nbsp;[Crates.io](https://crates.io/crates/rs-cache)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/7/74/Water_rune_detail.png/800px-Water_rune_detail.png?4e790" width="10"> &nbsp;[Documentation](https://docs.rs/rs-cache)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/e/ef/Nature_rune_detail.png/800px-Nature_rune_detail.png?a062f" width="10"> &nbsp;[Examples](examples/)

The public API of this crate is still evolving.
Currently supports both OSRS & RS3, although both are still limited.

The `osrs_specifications.md` & `rs3_specifications.md` documents (located in the root directory of this project) contain a detailed description of the design of the corresponding cache for educational purposes. Both documents are still a work in progress and are possibly incomplete.

Integration tests are running on Oldschool RuneScape revision 180, which you can run at any time because the cache is included in the `./data/osrs_cache` directory. RS3 Integration tests are running on revision 904. The RS3 cache is too large to include on GitHub.

The minimum supported `rustc` version is `1.41`.

This crate is passively maintained. Additional features will be implemented once they are needed for my own server.
The public API of this crate is still evolving due to constant discoveries and overhauls to improve the overal usage.
__If you require a certain feature feel free to open an issue.__

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rs-cache = "0.4"
```

Examples can be found in the [examples](examples/) directory. These examples include setting up your own custom cache or store and how the update protocol could be handled.

## Acknowledgements

The following sources aided with the development of this crate:\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/d/dc/Cosmic_rune_detail.png/800px-Cosmic_rune_detail.png?734d1" width="10"> &nbsp;[OpenRS](https://www.rune-server.ee/runescape-development/rs-503-client-server/downloads/312510-openrs-cache-library.html)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/f/f3/Air_rune_detail.png/800px-Air_rune_detail.png?b7f49" width="10"> &nbsp;[RuneLite](https://runelite.net/)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/0/0f/Law_rune_detail.png/800px-Law_rune_detail.png?dc1f3" width="10"> &nbsp;[OSRS Cache Parsing Blog](https://www.osrsbox.com/blog/2018/07/26/osrs-cache-research-extract-cache-definitions/)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/a/ae/Chaos_rune_detail.png/800px-Chaos_rune_detail.png?0d8cb" width="10"> &nbsp;[RSMod](https://github.com/Tomm0017/rsmod)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/8/8b/Soul_rune_detail.png/800px-Soul_rune_detail.png?75ada" width="10"> &nbsp;[Librsfs](https://github.com/Velocity-/librsfs)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/c/c1/Blood_rune_detail.png/800px-Blood_rune_detail.png?2cf9e" width="10"> &nbsp;[OSRSBox](https://www.osrsbox.com/)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/7/72/Earth_rune_detail.png/800px-Earth_rune_detail.png?991bd" width="10"> &nbsp;[Jagex-Store-5](https://github.com/guthix/Jagex-Store-5)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/b/bd/Body_rune_detail.png/800px-Body_rune_detail.png?9e79a" width="10"> &nbsp;[OldScape-Cache](https://github.com/guthix/OldScape-Cache)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/7/70/Wrath_rune.png?3a737" width="10"> &nbsp;[Matrix 876](https://www.rune-server.ee/runescape-development/rs-503-client-server/downloads/648085-matrix-3-876-recommended-876-rs3-server.html)


## License
RS-Cache is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.