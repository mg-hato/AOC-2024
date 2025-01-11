
pub fn inc(num: usize) -> Option<usize> { num.checked_add(1) }
pub fn same(num: usize) -> Option<usize> { Some(num) }
pub fn dec(num: usize) -> Option<usize> { num.checked_sub(1) }
