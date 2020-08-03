#![deny(clippy::all, clippy::nursery)]

mod cache;
mod cksm;
pub mod idx;
pub mod arc;
pub mod ext;
pub mod error;
pub mod store;
pub mod codec;
#[macro_use]
pub mod util;
pub mod def;
pub mod ldr;

pub type OsrsCache = Cache<store::MemoryStore>;
pub type Rs3Cache = Cache<store::FileStore>;

#[doc(inline)]
pub use error::Result;
#[doc(inline)]
pub use cache::{ Cache, CacheCore, CacheRead };
#[doc(inline)]
pub use store::Store;
#[doc(inline)]
pub use ldr::Loader;
#[doc(inline)]
pub use def::Definition;