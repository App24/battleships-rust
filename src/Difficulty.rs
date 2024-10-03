use colored::Color;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
}

impl Difficulty {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
            _ => "",
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Self::Easy => Color::Green,
            Self::Medium => Color::Yellow,
            Self::Hard => Color::Red,
        }
    }
}
