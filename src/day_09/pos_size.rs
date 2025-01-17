
/// A tuple carrying two numbers: position & size
/// But a bit more formalised than just a tuple
#[derive(Clone, Copy)]
pub struct PosSize((usize, usize));

impl PosSize {
    pub fn new(position: usize, size: usize) -> PosSize { PosSize((position, size)) }

    pub fn position(&self) -> usize {
        let PosSize((pos, _)) = *self;
        pos
    }

    pub fn size(&self) -> usize {
        let PosSize((_, size)) = *self;
        size
    }
}

/// Operation for segment tree on tuple of form (position, size).
/// Querying the segment tree we wish to find what is the greatest size spot we have in a certain range.
/// Thus, combine operation on two tuples LHS and RHS works as follows:
/// - if one has a greater size than the other, return the one with the greater size
/// - if both are of equal size, return the one with lesser position
pub struct PosSizeCombineOperation;

impl segment_tree::ops::Commutative<PosSize> for PosSizeCombineOperation {}

impl segment_tree::ops::Identity<PosSize> for PosSizeCombineOperation {
    fn identity(&self) -> PosSize { PosSize::new(0, 0) }
}

impl segment_tree::ops::Operation<PosSize> for PosSizeCombineOperation {
    fn combine(&self, a: &PosSize, b: &PosSize) -> PosSize {
        match a.size().cmp(&b.size()) {
            std::cmp::Ordering::Greater => *a,
            std::cmp::Ordering::Equal if a.position() <= b.position() => *a,
            _ => *b,
        }
    }
}