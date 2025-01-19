use super::{movement::Movement, position::UPosition};

pub trait Boundary {
    fn bound(&self, pos: UPosition) -> Option<UPosition>;
}

pub fn apply<B>(boundary: B, movement: Movement, position: UPosition) -> Option<UPosition> where B : Boundary {
    movement.apply(position).and_then(|pos|boundary.bound(pos))
}