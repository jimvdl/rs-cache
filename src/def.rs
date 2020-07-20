mod loader;
#[doc(hidden)]
pub mod item_def;
#[doc(hidden)]
pub mod npc_def;
#[doc(hidden)]
pub mod obj_def;

pub use loader::*;
pub use item_def::ItemDefinition;
pub use npc_def::NpcDefinition;
pub use obj_def::ObjectDefinition;

use std::{ io, marker::Sized };

mod internal {
    pub trait Sealed {}

    impl Sealed for super::ItemDefinition {}
    impl Sealed for super::NpcDefinition {}
    impl Sealed for super::ObjectDefinition {}

    impl Sealed for super::ItemLoader {}
    impl Sealed for super::NpcLoader {}
    impl Sealed for super::ObjectLoader {}
}

#[doc(hidden)]
pub trait Definition: internal::Sealed {
    fn new(id: u16, buffer: &[u8]) -> io::Result<Self> where Self: Sized;
}