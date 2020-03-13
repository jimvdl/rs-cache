#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Archive {
    pub sector: u32,
    pub length: usize
}

impl Archive {
    pub const fn new(sector: u32, length: usize) -> Self {
		Self { sector, length }
	}
}