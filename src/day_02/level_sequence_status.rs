/// Captures the idea of a growing sequence of levels and their safety status
#[derive(Clone)]
pub enum LevelSequenceStatus {
    /// Status for a sequence that is unsafe.
    /// Any other sequence expanded from an unsafe sequence will also be unsafe.
    Unsafe,

    /// Status for an empty sequence. **Safe**.
    NoData,

    /// Status for a single-level sequence. **Safe**.
    /// Captures the number of the sequence.
    /// No idea of increasing or decreasing trend has been established.
    Single(u32),

    /// Status for a **safe** sequence of at least two levels.
    /// The number represents the latest level in the sequence.
    /// The boolean represents whether the sequence is increasing (`true`)
    /// or decreasing (`false`).
    Trend(u32, bool),
}

impl LevelSequenceStatus {
    pub fn is_safe(&self) -> bool {
        match self {
            Self::Unsafe => false,
            _ => true,
        }
    }

    /// Returns the option on level, described by the status enum:
    /// - For status Single and Trend: returns the `Some(level)`
    /// - Otherwise, returns `None`
    pub fn level(&self) -> Option<u32> {
        match self {
            Self::Single(level) => Some(*level),
            Self::Trend(level, _) => Some(*level),
            _ => None,
        }
    }

    /// Returns an option on boolean describing the movements. `true` for increasing
    /// and `false` for decreasing:
    /// - If status is `Trend(level, b)` returns `Some(b)`
    /// - Otherwise, returns `None`
    pub fn trend_boolean(&self) -> Option<bool> {
        match self {
            Self::Trend(_, is_increasing) => Some(*is_increasing),
            _ => None,
        }
    }
}