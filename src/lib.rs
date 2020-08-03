#![deny(clippy::all, clippy::nursery)]

#![warn(
    clippy::clone_on_ref_ptr, 
    clippy::redundant_clone, 
    clippy::default_trait_access, 
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_into_iter_loop, 
    clippy::explicit_iter_loop, 
    clippy::filter_map, 
    clippy::filter_map_next, 
    clippy::find_map, 
    clippy::get_unwrap,
    clippy::items_after_statements, 
    clippy::large_digit_groups, 
    clippy::map_flatten, 
    clippy::match_same_arms, 
    clippy::maybe_infinite_iter, 
    clippy::mem_forget,
    clippy::missing_inline_in_public_items, 
    clippy::multiple_inherent_impl, 
    clippy::mut_mut, 
    clippy::needless_continue,
    clippy::needless_pass_by_value, 
    clippy::map_unwrap_or, 
    clippy::pub_enum_variant_names, 
    clippy::unused_self, 
    clippy::similar_names, 
    clippy::single_match_else, 
    clippy::too_many_lines, 
    clippy::type_repetition_in_bounds,
    clippy::unseparated_literal_suffix, 
    clippy::used_underscore_binding
)]

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