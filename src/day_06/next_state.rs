use super::guard_state::GuardState;


/// A concept that represents "next state" in the guard's patrol. It can be one of the following:
/// - Next: a standard most common option. A guard state with guard's position and currently facing direction
/// - Out: the state achieved when the guard leaves the mapped area.
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum NextState {
    Next(GuardState),
    Out,
}
