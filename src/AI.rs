use crate::{Board::Board, Difficulty::Difficulty, Ship::ShipType};
use rand::prelude::*;

pub struct AI {
    hit_spots: Vec<u32>,
    attack_spots: Vec<u32>,
}

impl AI {
    pub fn new() -> Self {
        Self {
            attack_spots: Vec::new(),
            hit_spots: Vec::new(),
        }
    }

    pub fn place_ships(&self, ai_board: &mut Board) {
        let mut current_ship_type = <ShipType as num::FromPrimitive>::from_u32(1).unwrap();
        let mut rng = rand::thread_rng();
        for i in 0..5 {
            let current_ship_type_int = num::ToPrimitive::to_u32(&current_ship_type).unwrap();

            let ship_size: u32 = current_ship_type.get_size() as u32;

            loop {
                let hor_pos = rng.gen_range(0..=(ai_board.get_board_size() - ship_size));
                let ver_pos = rng.gen_range(0..ai_board.get_board_size());

                let horizontal = rng.gen_bool(0.5);

                let x = if horizontal { hor_pos } else { ver_pos };
                let y = if horizontal { ver_pos } else { hor_pos };

                let previous_ship_types = ai_board.get_ship_types(x, y, ship_size, horizontal);
                ai_board.set_ship_type(x, y, current_ship_type, ship_size, horizontal);

                if !ai_board.any_overlap() {
                    break;
                }

                ai_board.set_ship_types(x, y, &previous_ship_types, horizontal);
            }

            current_ship_type =
                <ShipType as num::FromPrimitive>::from_u32(current_ship_type_int + 1).unwrap();
        }
    }

    pub fn do_turn(&mut self, player_board: &mut Board, game_difficulty: Difficulty) {
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        let mut rng = rand::thread_rng();

        match game_difficulty {
            Difficulty::Easy => {
                let pos = self.random_turn(player_board);
                x = pos[0];
                y = pos[1];
            }
            Difficulty::Medium => {
                let option = rng.gen_range(0..=1);
                match option {
                    0 => {
                        let pos = self.random_turn(player_board);
                        x = pos[0];
                        y = pos[1];
                    }
                    1 | _ => {
                        let pos = self.smart_turn(player_board);
                        x = pos[0];
                        y = pos[1];
                    }
                }
            }
            Difficulty::Hard => {
                let option = rng.gen_range(0..=3);
                match option {
                    0 => {
                        let pos = self.corner_turn(player_board);
                        x = pos[0];
                        y = pos[1];
                    }
                    1 => {
                        let pos = self.center_turn(player_board);
                        x = pos[0];
                        y = pos[1];
                    }
                    2 => {
                        let pos = self.smart_turn(player_board);
                        x = pos[0];
                        y = pos[1];
                    }
                    3 | _ => {
                        let pos = self.smart_turn(player_board);
                        x = pos[0];
                        y = pos[1];
                    }
                }
            }
        }

        if x >= player_board.get_board_size() || y >= player_board.get_board_size() {
            self.do_turn(player_board, game_difficulty);
            return;
        }

        if self.has_attack(x, y) {
            self.do_turn(player_board, game_difficulty);
            return;
        }

        if player_board.get_ship(x, y).unwrap().damaged {
            self.do_turn(player_board, game_difficulty);
            return;
        }

        if player_board.hit_ship(x, y) {
            self.hit_spots.push(x);
            self.hit_spots.push(y);
        }

        self.attack_spots.push(x);
        self.attack_spots.push(y);
    }

    fn random_turn(&mut self, player_board: &Board) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        vec![
            rng.gen_range(0..player_board.get_board_size()),
            rng.gen_range(0..player_board.get_board_size()),
        ]
    }

    fn corner_turn(&mut self, player_board: &Board) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=1) * (player_board.get_board_size() - 1);
        let y = rng.gen_range(0..=1) * (player_board.get_board_size() - 1);

        vec![x, y]
    }

    fn center_turn(&mut self, player_board: &Board) -> Vec<u32> {
        let middle = player_board.get_board_size() / 2;
        let width = player_board.get_board_size() / 4;

        let mut rng = rand::thread_rng();

        let x = rng.gen_range(width..=(width + middle));
        let y = rng.gen_range(width..=(width + middle));

        vec![x, y]
    }

    fn smart_turn(&mut self, player_board: &Board) -> Vec<u32> {
        if self.hit_spots.len() <= 0 {
            return self.random_turn(player_board);
        }

        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0..(self.hit_spots.len() / 2)) * 2;

        let hit_x = self.hit_spots[index];
        let hit_y = self.hit_spots[index + 1];

        let option = rng.gen_range(0..8);

        let mut x: u32 = 0;
        let mut y: u32 = 0;

        match option {
            0 => {
                x = hit_x - 1;
                y = hit_y - 1;
            }
            1 => {
                x = hit_x;
                y = hit_y - 1;
            }
            2 => {
                x = hit_x + 1;
                y = hit_y - 1;
            }
            3 => {
                x = hit_x - 1;
                y = hit_y;
            }
            4 => {
                x = hit_x + 1;
                y = hit_y;
            }
            5 => {
                x = hit_x - 1;
                y = hit_y + 1;
            }
            6 => {
                x = hit_x;
                y = hit_y + 1;
            }
            7 | _ => {
                x = hit_x + 1;
                y = hit_y + 1;
            }
        }

        if self.has_attack(x, y) {
            return self.smart_turn(player_board);
        }

        vec![x, y]
    }

    fn has_attack(&self, x: u32, y: u32) -> bool {
        if self.attack_spots.len() <= 0 {
            return false;
        }

        let length = self.attack_spots.len() / 2;

        for i in (0..length).step_by(2) {
            let attack_x = self.attack_spots[i];
            let attack_y = self.attack_spots[i + 1];
            if attack_x == x && attack_y == y {
                return true;
            }
        }

        false
    }
}
