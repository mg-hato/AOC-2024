
use super::{compact::Compact, memory_block::MemoryBlock, pos_size::{PosSize, PosSizeCombineOperation}};

/// Compacter that moves whole block sequence of a file into empty space
pub struct FileByFileCompacter;

impl FileByFileCompacter {
    fn as_vec_memory_block(mut pos_block: Vec<(usize, MemoryBlock)>) -> Vec<MemoryBlock> {
        pos_block.sort_by_key(|&(position,_)|position);
        let mut pos = 0;
        let mut rtn = vec![];
        for (position, block) in pos_block {
            rtn.push(MemoryBlock::new(position - pos, None));
            rtn.push(block);
            pos = position + block.size;
        }
        rtn
    }
}

impl Compact for FileByFileCompacter {
    fn compact(&self, mem: Vec<MemoryBlock>) -> Vec<MemoryBlock> {
        let mut files = vec![];
        let mut empty_blocks = vec![];

        let mut position = 0;
        for block in mem {
            if block.size == 0 { continue; }
            match  block.id {
                Some(_) => { files.push((position, block)); },
                None => empty_blocks.push(PosSize::new(position, block.size)),
            }
            position += block.size;
        }

        let mut seg_tree = segment_tree::SegmentPoint::build(empty_blocks, PosSizeCombineOperation);

        let mut blocks_reordered = vec![];
        for (block_position, block) in files.into_iter().rev() {
            let (mut left, mut right) = (0, seg_tree.len());
            // INV: 
            // 1. for b in emtpy_blocks[0..left): b.size < block.size
            // 2. 0 <= left <= right <= length(empty_blocks)
            while left + 1 < right {
                let mid = (left + right) / 2;
                if seg_tree.query(left, mid).size() < block.size {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            let chosen_empty = seg_tree.query(left, right);
            let pos_block_final = if chosen_empty.size() >= block.size && chosen_empty.position() < block_position {
                let new_empty_size = chosen_empty.size() - block.size;
                let new_empty_position = chosen_empty.position() + block.size;
                seg_tree.modify(left, PosSize::new(new_empty_position, new_empty_size));
                (chosen_empty.position(), block)
            } else { (block_position, block) };
            blocks_reordered.push(pos_block_final);
        }
        Self::as_vec_memory_block(blocks_reordered)
    }
}
