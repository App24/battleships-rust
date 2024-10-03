use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy)]
pub struct Ship {
    pub ship_type: ShipType,
    pub damaged: bool,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            ship_type: ShipType::Empty,
            damaged: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ShipType {
    Empty = 0,
    Destroyer = 1,
    Submarine = 2,
    Cruiser = 3,
    Battleship = 4,
    Carrier = 5,
    Shot = 6,
    Selection = 7,
    WaterSelection = 8,
}

impl ShipType {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Destroyer => "Destroyer",
            Self::Submarine => "Submarine",
            Self::Cruiser => "Cruiser",
            Self::Battleship => "Battleship",
            Self::Carrier => "Carrier",
            _ => "",
        }
    }

    pub fn to_shorten(&self) -> &str {
        match self {
            Self::Destroyer => "des",
            Self::Submarine => "sub",
            Self::Cruiser => "cru",
            Self::Battleship => "bat",
            Self::Carrier => "car",
            Self::Shot => " x ",
            Self::Selection => " + ",
            Self::WaterSelection => " + ",
            _ => "",
        }
    }

    pub fn get_size(&self) -> u32 {
        match self {
            Self::Destroyer => 2,
            Self::Submarine => 3,
            Self::Cruiser => 3,
            Self::Battleship => 4,
            Self::Carrier => 5,
            _ => 0,
        }
    }
}
