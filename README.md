# rs-cache

[![Build](https://github.com/jimvdl/rs-cache/workflows/build/badge.svg)](https://github.com/jimvdl/rs-cache)
[![API](https://docs.rs/rs-cache/badge.svg)](https://docs.rs/rs-cache)
[![Crate](https://img.shields.io/crates/v/rs-cache)](https://crates.io/crates/rs-cache)
[![dependency status](https://deps.rs/repo/github/jimvdl/rs-cache/status.svg)](https://deps.rs/repo/github/jimvdl/rs-cache)
[![OSRS Version](https://img.shields.io/badge/OSRS-180-blue)]()
[![RS3 Version](https://img.shields.io/badge/RS3-904-blue)]()

A read-only, high-level, virtual file API for the RuneScape cache.

This crate provides high performant data reads into the [Oldschool RuneScape](https://oldschool.runescape.com/) and [RuneScape 3](https://www.runescape.com/) cache file systems. It can read the necessary data to synchronize the client's cache with the server. There are also some loaders that give access to definitions from the cache such as items or npcs. 

For read-heavy workloads, a writer can be used to prevent continuous buffer allocations.
By default every read will allocate a writer with the correct capacity.

RuneScape’s chat system uses `Huffman` encoding to compress messages; this library contains a huffman implementation to decompress these messages.

When a RuneScape client sends game packets the id's are encoded and can be decoded with the `IsaacRand`
implementation. These id's are encoded by the client in a predictable random order which can be reversed if
the server has its own `IsaacRand` with the same encoder/decoder keys. These keys are sent by the client
on login and are user specific. It will only send encoded packet id's if the packets are game packets.

Note that this crate is still evolving; both OSRS & RS3 are not fully supported/implemented and
will probably contain bugs or miss core features. If you require features or find bugs consider [opening
an issue](https://github.com/jimvdl/rs-cache/issues/new).

Useful links:\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/5/5d/Fire_rune_detail.png/800px-Fire_rune_detail.png?07ed5" width="10"> &nbsp;[Releases](https://github.com/jimvdl/rs-cache/tags)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/7/74/Water_rune_detail.png/800px-Water_rune_detail.png?4e790" width="10"> &nbsp;[Documentation](https://docs.rs/rs-cache)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="https://oldschool.runescape.wiki/images/thumb/e/ef/Nature_rune_detail.png/800px-Nature_rune_detail.png?a062f" width="10"> &nbsp;[Examples](examples/)

## Safety

In order to read bytes in a high performant way the cache uses [memmap2](https://crates.io/crates/memmap2). This can be unsafe because of its potential for _Undefined Behaviour_ when the underlying file is subsequently modified, in or out of process. Using `Mmap` here is safe because the RuneScape cache is a read-only binary file system. The map will remain valid even after the `File` is dropped, it's completely independent of the `File` used to create it. Therefore, the use of unsafe is not propagated outwards. When the `Cache` is dropped memory will be subsequently unmapped.

## Features

The cache's protocol defaults to OSRS. In order to use the RS3 protocol you can enable the `rs3` feature flag.
A lot of types derive [serde](https://crates.io/crates/serde)'s `Serialize` and `Deserialize`. The `serde` feature flag can be used to enable (de)serialization on any compatible types.

## Quick Start

For an instance that stays local to this thread you can simply use:
```rust
use rscache::Cache;

let cache = Cache::new("./data/osrs_cache").unwrap();

let index_id = 2; // Config index.
let archive_id = 10; // Archive containing item definitions.

let buffer = cache.read(index_id, archive_id).unwrap();
```

If you want to share the instance over multiple threads you can do so by wrapping it in an [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
```rust
use rscache::Cache;
use std::sync::Arc;

let cache = Arc::new(Cache::new("./data/osrs_cache").unwrap());

let c = Arc::clone(&cache);
std::thread::spawn(move || {
    c.read(0, 10).unwrap();
});

std::thread::spawn(move || {
    cache.read(0, 10).unwrap();
});
```

The recommended usage would be to wrap it using [`lazy_static`](https://docs.rs/lazy_static/latest/lazy_static/) making it the easiest way to access cache data from anywhere and at any time. No need for an `Arc` or a `Mutex` because `Cache` will always be `Send` & `Sync`.
```rust
use rscache::Cache;
use once_cell::sync::Lazy;

static CACHE: Lazy<Cache> = Lazy::new(|| {
    Cache::new("./data/osrs_cache").unwrap()
});

std::thread::spawn(move || {
    CACHE.read(0, 10).unwrap();
});

std::thread::spawn(move || {
    CACHE.read(0, 10).unwrap();
});
```

Integration tests are running on Oldschool RuneScape version 180, which you can run at any time because the cache is included in the `./data/osrs_cache` directory. RS3 Integration tests are running on version 904. The RS3 cache is too large to include on GitHub.

This crate is marked as experimental. I will implement additional features once I need them for my own project.
__If you require a certain feature feel free to [open an issue](https://github.com/jimvdl/rs-cache/issues/new).__

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rs-cache = "0.8.4"
```

Examples can be found in the [examples](examples/) directory which include both update protocols.

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
`rs-cache` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.