#![warn(clippy::all, clippy::nursery, clippy::clone_on_ref_ptr, clippy::redundant_clone, clippy::default_trait_access, clippy::expl_impl_clone_on_copy,
    clippy::explicit_into_iter_loop, clippy::explicit_iter_loop, clippy::filter_map, clippy::filter_map_next, clippy::find_map, clippy::get_unwrap,
    clippy::items_after_statements, clippy::large_digit_groups, clippy::map_flatten, clippy::match_same_arms, clippy::maybe_infinite_iter, clippy::mem_forget,
    clippy::missing_inline_in_public_items, clippy::multiple_inherent_impl, clippy::mut_mut, clippy::needless_continue,
    clippy::needless_pass_by_value, clippy::option_map_unwrap_or, clippy::option_map_unwrap_or_else, clippy::pub_enum_variant_names, clippy::unused_self,
    clippy::result_map_unwrap_or_else, clippy::similar_names, clippy::single_match_else, clippy::too_many_lines, clippy::type_repetition_in_bounds,
    clippy::unseparated_literal_suffix, clippy::used_underscore_binding)]

#![allow(clippy::suspicious_else_formatting)]

mod cache;
mod checksum;
mod errors;
mod container;
mod traits;

pub use cache::Cache;
pub use checksum::Checksum;
pub use errors::*;
pub use traits::*;
use container::{ Container, CompressionType };