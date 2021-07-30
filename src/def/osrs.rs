#[allow(clippy::too_many_lines)]
mod item_def;
mod npc_def;
#[allow(clippy::too_many_lines)]
mod obj_def;
mod map_def;
#[allow(clippy::too_many_lines)]
#[allow(clippy::cognitive_complexity)]
mod mdl_def;

pub use item_def::*;
pub use npc_def::*;
pub use obj_def::*;
pub use map_def::*;
pub use mdl_def::*;