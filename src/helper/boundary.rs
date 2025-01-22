use super::{movement::Movement, position::UPosition};

pub trait Boundary {
    fn bound(&self, pos: UPosition) -> Option<UPosition>;

    fn apply(&self, movement: Movement, position: UPosition) -> Option<UPosition> {
        movement.apply(position).and_then(|pos|self.bound(pos))
    }
}

pub fn apply<B>(boundary: B, movement: Movement, position: UPosition) -> Option<UPosition> where B : Boundary {
    movement.apply(position).and_then(|pos|boundary.bound(pos))
}