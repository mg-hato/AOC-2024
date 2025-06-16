use std::fmt::Display;

use crate::helper::display::vector_display;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocalNetwork(pub Vec<Connection>);

impl Display for LocalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let LocalNetwork(network) = self;
        write!(f, "[{}]", vector_display(network, ", "))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Connection(pub String, pub String);

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Connection(left_computer, right_computer) = self;
        write!(f, "{}-{}", left_computer, right_computer)
    }
}