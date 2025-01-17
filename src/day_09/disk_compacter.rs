use crate::{answer::{Answer, DisplayableAnswer}, helper::option::pair_merge, solver::Solve};

use super::{compact::Compact, memory_block::MemoryBlock, model::DiskMap};

mod error {
    use crate::{day_09::memory_block::MemoryBlock, helper::display::vector_display};

    use super::Accumulator;

    const PREFIX: &str = "[Solver D-09]";
    pub fn overflow_error(accumulator: Accumulator, mem_block: MemoryBlock) -> String {
        vector_display(&vec![
            format!("{} an overflow error occurred while calculating check sum with", PREFIX),
            format!("accumulator: {:?}; memory block: {:?}", accumulator, mem_block),
        ], " ")
    }
}

pub struct DiskCompacter {
    compacter: Box<dyn Compact>,
}

impl DiskCompacter {
    pub fn new<CompactProc>(compacter_procedure: CompactProc) -> DiskCompacter where CompactProc: Compact + 'static {
        DiskCompacter { compacter: Box::new(compacter_procedure) }
    }

    fn as_memory_block_vector(disk_map: DiskMap) -> Vec<MemoryBlock> {
        let mut vec = vec![];
        let mut is_file = true;
        let mut id = 0;

        let DiskMap(disk) = disk_map;
        for size in disk {
            let id = match is_file {
                true  => { id += 1; Some(id - 1) },
                false => None,
            };
            is_file = !is_file;

            // Add non-zero memory blocks
            if size > 0 { vec.push(MemoryBlock::new(size, id)); }
        }

        vec
    }

    fn check_sum_step(accumulator: Accumulator, mem_block: MemoryBlock) -> Result<Accumulator, String> {
        
        let size = mem_block.size;
        let pos = accumulator.last_position;
        let new_position = pos + size;
        
        // Early return: if memory block is empty or length of the block is 0, accumulator's value won't change
        if mem_block.id.is_none() || mem_block.size == 0 {
            return Ok(Accumulator { last_position: new_position, ..accumulator });
        }
        let id = mem_block.id.unwrap();
        
        // Calculate: increment =
        // = id * SUM(pos + [0], pos + [1], ... , pos + [size - 1])
        // = id * (pos * size + SUM(0, 1, ... , size - 1))
        // = id * ((pos * size) + ((size - 1) * size / 2))
        // = id * (rectangle_component + triangle_component)

        let triangle_component = size.checked_mul(size - 1).and_then(|n|n.checked_div(2));
        let rectangle_component = pos.checked_mul(size);
        let increment =  pair_merge(triangle_component, rectangle_component)
            .and_then(|(lhs, rhs)|lhs.checked_add(rhs))
            .and_then(|n|n.checked_mul(id));

        increment.and_then(|i|i.checked_add(accumulator.value))
            .map(|value|Accumulator{ value, last_position: new_position })
            .ok_or_else(||error::overflow_error(accumulator, mem_block))
    }
}

impl Solve<DiskMap> for DiskCompacter {
    fn solve(&self, input: DiskMap) -> Result<Answer, String> {
        self.compacter
            .compact(Self::as_memory_block_vector(input))
            .into_iter().try_fold(Accumulator{ value: 0, last_position: 0 }, Self::check_sum_step)
            .map(|acc|DisplayableAnswer::new(acc.value))
    }
}

#[derive(Debug)]
struct Accumulator {
    value: usize,
    last_position: usize,
}