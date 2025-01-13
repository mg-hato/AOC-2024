use super::{compact::Compact, memory_block::MemoryBlock};

pub struct BlockByBlockCompacter;

/// Compacter that moves block by block into empty spaces
impl BlockByBlockCompacter {
    /// Expand each memory block into unit memory blocks
    fn expand(mem: Vec<MemoryBlock>) -> Vec<Option<usize>> {
        mem.into_iter()
            .flat_map(|block|(0..block.size).map(move |_|block.id))
            .collect()
    }
}

impl Compact for BlockByBlockCompacter {
    fn compact(&self, mem: Vec<MemoryBlock>) -> Vec<MemoryBlock> {
        let mut memory = Self::expand(mem);

        // Invariant:
        // Let n be the size of `memory`
        // 1. 0 <= left, right < n and left <= right
        // 2. memory[0..left) is Some(*) (used memory block)
        // 3. memory(right..n) is None (free memory block)
        let mut left = 0;
        let mut right = memory.len() - 1;
        while left < right {
            while left < right && memory[left].is_some() { left += 1; }
            while left < right && memory[right].is_none() { right -= 1; }
            if left < right {
                memory[left] = memory[right];
                memory[right] = None;
            }
        }
        
        memory.into_iter().map(|id|MemoryBlock::new(1, id)).collect()
    }
}