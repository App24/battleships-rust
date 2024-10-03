use crate::Ship::{Ship, ShipType};
use colored::{Color, Colorize};

pub struct Board {
    board_size: u32,
    ships: Vec<Ship>,
}

impl Board {
    pub fn new(board_size: Option<u32>) -> Self {
        let board_size = board_size.unwrap_or(10);
        let length = board_size * board_size;
        let ships: Vec<Ship> = vec![Ship::new(); length as usize];
        Self { board_size, ships }
    }

    pub fn get_ships(&self) -> &Vec<Ship> {
        &self.ships
    }

    pub fn get_ship(&mut self, x: u32, y: u32) -> Result<&mut Ship, &'static str> {
        let index = y * self.board_size + x;
        match x >= self.board_size || y >= self.board_size {
            true => Err("Position is not on the board"),
            false => Ok(self.ships.get_mut(index as usize).unwrap()),
        }
    }

    pub fn get_board_size(&self) -> u32 {
        self.board_size
    }

    pub fn any_overlap(&self) -> bool {
        let ships = self.get_ships();
        for ship in ships {
            if ship.damaged {
                return true;
            }
        }
        false
    }

    pub fn all_destroyed(&self) -> bool {
        let ships = self.get_ships();
        for ship in ships {
            match ship.ship_type {
                ShipType::Empty | ShipType::Selection | ShipType::WaterSelection => continue,
                _ => {}
            }
            if !ship.damaged {
                return false;
            }
        }
        true
    }

    pub fn show(&mut self) {
        for y in -1..(self.board_size as i32) {
            for x in -1..(self.board_size as i32) {
                if x == -1 && y == -1 {
                    print!("  |");
                } else if x == -1 && y >= 0 {
                    let line = y + 1;
                    print!("{}", line.to_string().yellow());
                    if line < 10 {
                        print!(" ");
                    }
                    print!("|");
                } else if y == -1 && x >= 0 {
                    print!(" {}", ((65 + x) as u8 as char).to_string().blue());
                    print!(" |");
                } else {
                    let ship = self.get_ship(x as u32, y as u32).unwrap();
                    let repr = ship.ship_type.to_shorten();
                    let mut background_color: Color = Color::Black;
                    let mut front_color: Color = Color::White;
                    if ship.damaged {
                        match ship.ship_type {
                            ShipType::Selection | ShipType::WaterSelection => {
                                background_color = Color::TrueColor {
                                    r: 128,
                                    g: 128,
                                    b: 128,
                                }
                            }
                            _ => {
                                background_color = Color::TrueColor {
                                    r: 128,
                                    g: 128,
                                    b: 128,
                                };
                                front_color = Color::Red;
                            }
                        }
                    } else {
                        match ship.ship_type {
                            ShipType::Empty | ShipType::WaterSelection | ShipType::Shot => {
                                background_color = Color::TrueColor { r: 0, g: 0, b: 128 }
                            }
                            _ => {
                                background_color = Color::TrueColor {
                                    r: 128,
                                    g: 128,
                                    b: 128,
                                }
                            }
                        }
                    }
                    print!("{}", repr.on_color(background_color).color(front_color));

                    if repr.len() < 3 {
                        let diff = 3 - repr.len();
                        for _ in 0..diff {
                            print!("{}", " ".color(front_color).on_color(background_color));
                        }
                    }

                    front_color = Color::White;
                    if x < (self.board_size as i32) - 1 {
                        background_color = Color::TrueColor { r: 0, g: 0, b: 128 }
                    } else {
                        background_color = Color::Black;
                    }

                    print!("{}", "|".color(front_color).on_color(background_color));
                }
            }
            println!();
        }
    }

    pub fn hit_ship(&mut self, x: u32, y: u32) -> bool {
        let ship = match self.get_ship(x, y) {
            Ok(ship) => ship,
            Err(e) => {
                println!("{}", e);
                return false;
            }
        };

        if ship.ship_type == ShipType::Empty {
            return false;
        }
        if ship.damaged {
            return false;
        }
        ship.damaged = true;
        true
    }

    pub fn update_ship_selection(&mut self, previous_ship_type: &mut ShipType, x: u32, y: u32) {
        let ship = match self.get_ship(x, y) {
            Ok(ship) => ship,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        *previous_ship_type = ship.ship_type.clone();
        if *previous_ship_type == ShipType::Empty
            || (*previous_ship_type == ShipType::Shot && !ship.damaged)
        {
            ship.ship_type = ShipType::WaterSelection;
        } else {
            ship.ship_type = ShipType::Selection;
        }
    }

    pub fn set_ship_type(
        &mut self,
        x: u32,
        y: u32,
        ship_type: ShipType,
        amount: u32,
        horizontal: bool,
    ) {
        for i in 0..amount {
            let ship = self
                .get_ship(
                    x + match horizontal {
                        true => i,
                        false => 0,
                    },
                    y + match horizontal {
                        false => i,
                        true => 0,
                    },
                )
                .unwrap();
            if ship.ship_type != ship_type && ship.ship_type != ShipType::Empty {
                ship.damaged = true;
            } else {
                ship.damaged = false;
            }
            ship.ship_type = ship_type;
        }
    }

    pub fn set_ship_types(&mut self, x: u32, y: u32, ship_types: &Vec<ShipType>, horizontal: bool) {
        for i in 0..ship_types.len() {
            let ship = self
                .get_ship(
                    x + match horizontal {
                        true => i as u32,
                        false => 0,
                    },
                    y + match horizontal {
                        false => i as u32,
                        true => 0,
                    },
                )
                .unwrap();
            ship.damaged = false;
            ship.ship_type = ship_types[i];
        }
    }

    pub fn get_ship_types(&mut self, x: u32, y: u32, amount: u32, horizontal: bool) -> Vec<ShipType> {
        let mut ship_types:Vec<ShipType>=Vec::new();
        for i in 0..amount{
            ship_types.push(self.get_ship(x + match horizontal {
                true => i as u32,
                false => 0,
            },
            y + match horizontal {
                false => i as u32,
                true => 0,
            }).unwrap().ship_type.clone());
        }
        ship_types
    }
}
