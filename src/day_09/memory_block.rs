#[derive(Clone, Copy, Debug)]
pub struct MemoryBlock {
    pub size: usize,

    /// It is `None` for free memory, `Some(id)` for a file block 
    pub id: Option<usize>,
}

impl MemoryBlock {
    pub fn new(size: usize, id: Option<usize>) -> MemoryBlock {
        MemoryBlock { size, id }
    }
}