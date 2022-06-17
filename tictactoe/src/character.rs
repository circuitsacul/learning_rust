#[derive(Debug, PartialEq, Clone)]
pub enum Character {
    X,
    O,
    E,
}

impl Character {
    pub fn print(&self) {
        match self {
            Character::X => print!(" X "),
            Character::O => print!(" O "),
            Character::E => print!(" E "),
        }
    }
}
