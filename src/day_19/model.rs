use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TowelPatternsAndDesigns {
    pub patterns: Vec<String>,
    pub designs: Vec<String>,
}

impl Display for TowelPatternsAndDesigns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let TowelPatternsAndDesigns { patterns, designs } = self;
        write!(f, "{{ Patterns: [{}], Designs: [{}] }}", vector_display(patterns, ", "), vector_display(designs, ", "))
    }
}