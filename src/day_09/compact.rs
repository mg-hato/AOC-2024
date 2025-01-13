use super::memory_block::MemoryBlock;

/// Memory compacting procedure
pub trait Compact {
    // Returns compacted memory of `mem`. 
    fn compact(&self, mem: Vec<MemoryBlock>) -> Vec<MemoryBlock>;
}