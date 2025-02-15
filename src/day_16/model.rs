use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    Wall,
    Empty,
    Start,
    End,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Field::Wall  => '#',
            Field::Empty => '.',
            Field::Start => 'S',
            Field::End   => 'E',
        })
    }
}