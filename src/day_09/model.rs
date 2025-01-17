use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DiskMap(pub Vec<usize>);

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DiskMap(disk) = self;
        write!(f, "[{}]", vector_display(disk, ""))
    }
}