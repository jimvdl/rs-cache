use std::collections::LinkedList;

pub trait LinkedListExt {
    fn to_vec(&self) -> Vec<u8>;
}

impl LinkedListExt for LinkedList<&[u8]> {
    #[inline]
	fn to_vec(&self) -> Vec<u8> {
		let size = self.iter()
            .map(|data_block| data_block.len())
            .sum();

        let mut buffer = vec![0; size];
        let mut current = 0;

        for data_block in self {
            buffer[current..current + data_block.len()].copy_from_slice(data_block);
            current += data_block.len();
        }

        buffer
	}
}