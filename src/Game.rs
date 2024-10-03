use std::char::from_u32;

use colored::{Color, Colorize};
use getch_rs::{Getch, Key};

use crate::{Board::Board, Difficulty::Difficulty, Ship::ShipType, AI::AI};

pub struct Game {
    player_board: Board,
    ai_board: Board,
    guess_board: Board,
    ai: AI,
    game_difficulty: Difficulty,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player_board: Board::new(None),
            ai_board: Board::new(None),
            guess_board: Board::new(None),
            ai: AI::new(),
            game_difficulty: Difficulty::Easy,
        }
    }

    pub fn start_game(&mut self) {
        self.set_difficulty();
    }

    fn set_difficulty(&mut self) {
        let mut current_difficulty = <Difficulty as num::FromPrimitive>::from_u32(0).unwrap();
        let mut finished = false;

        let g = Getch::new();

        while !finished {
            let current_difficulty_int = num::ToPrimitive::to_u32(&current_difficulty).unwrap();
            println!(
                "Current Difficulty: {}",
                current_difficulty
                    .to_string()
                    .color(current_difficulty.get_color())
            );

            println!("Change Selection: Arrow Keys");
            println!("Confirm Selection: Enter Key");

            match g.getch() {
                Ok(Key::Left) => {
                    if current_difficulty_int > 0 {
                        current_difficulty = <Difficulty as num::FromPrimitive>::from_u32(
                            current_difficulty_int - 1,
                        )
                        .unwrap();
                    }
                }
                Ok(Key::Right) => {
                    if current_difficulty_int < 2 {
                        current_difficulty = <Difficulty as num::FromPrimitive>::from_u32(
                            current_difficulty_int + 1,
                        )
                        .unwrap();
                    }
                }
                Ok(Key::Char('\r')) => {
                    self.game_difficulty = current_difficulty;
                    finished = true;
                }
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
            print!("{}[2J", 27 as char);
        }

        self.setup_board();
    }

    fn setup_board(&mut self) {
        let g = Getch::new();

        self.ai.place_ships(&mut self.ai_board);

        let mut x: u32 = 0;
        let mut y: u32 = 0;

        let mut select_mode = true;

        let mut remaining_ships = vec![0, 1, 1, 1, 1, 1];

        let mut horizontal = true;

        let mut fininished = false;

        let mut current_ship_type = <ShipType as num::FromPrimitive>::from_u32(1).unwrap();

        let mut previous_ship_types: Vec<ShipType> = Vec::new();

        while !fininished {
            let current_ship_type_int = num::ToPrimitive::to_usize(&current_ship_type).unwrap();
            let mut color = Color::White;
            if remaining_ships[current_ship_type_int] <= 0 {
                color = Color::Red;
            } else if select_mode {
                color = Color::Green;
            }
            println!("{}", current_ship_type.to_string().color(color));

            println!(
                "{} {}",
                ((65 + x) as u8 as char).to_string().color(Color::Blue),
                (y + 1).to_string().color(Color::Yellow)
            );

            self.player_board.show();

            println!();

            println!("Change/Move Selection: Arrow Keys");
            println!("Rotate: R Key");
            println!("Confirm Selection: Enter Key");
            println!("Go Back/Finish: Escape Key");

            let c = g.getch();

            if select_mode {
                match c {
                    Ok(Key::Left) => {
                        if current_ship_type_int > 1 {
                            current_ship_type = <ShipType as num::FromPrimitive>::from_usize(
                                current_ship_type_int - 1,
                            )
                            .unwrap();
                        }
                    }
                    Ok(Key::Right) => {
                        if current_ship_type_int < 5 {
                            current_ship_type = <ShipType as num::FromPrimitive>::from_usize(
                                current_ship_type_int + 1,
                            )
                            .unwrap();
                        }
                    }
                    Ok(Key::Char('\r')) => {
                        if remaining_ships[current_ship_type_int] > 0 {
                            select_mode = false;
                            previous_ship_types = self.player_board.get_ship_types(
                                x,
                                y,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                            self.player_board.set_ship_type(
                                x,
                                y,
                                current_ship_type,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                        }
                    }
                    Ok(Key::Esc) => {
                        fininished = true;
                        for i in 0..6 {
                            if remaining_ships[i] > 0 {
                                fininished = false;
                                break;
                            }
                        }
                    }
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            } else {
                match c {
                    Ok(Key::Char('r')) => {
                        self.player_board
                            .set_ship_types(x, y, &previous_ship_types, horizontal);
                        horizontal = !horizontal;
                        if horizontal
                            && x >= (self.player_board.get_board_size()
                                - current_ship_type.get_size())
                        {
                            x = self.player_board.get_board_size() - current_ship_type.get_size();
                        } else if !horizontal
                            && y >= self.player_board.get_board_size()
                                - current_ship_type.get_size()
                        {
                            y = self.player_board.get_board_size() - current_ship_type.get_size();
                        }
                        previous_ship_types = self.player_board.get_ship_types(
                            x,
                            y,
                            current_ship_type.get_size(),
                            horizontal,
                        );
                        self.player_board.set_ship_type(
                            x,
                            y,
                            current_ship_type,
                            current_ship_type.get_size(),
                            horizontal,
                        );
                    }
                    Ok(Key::Esc) => {
                        select_mode = true;
                        self.player_board
                            .set_ship_types(x, y, &previous_ship_types, horizontal);
                    }
                    Ok(Key::Char('\r')) => {
                        if !self.player_board.any_overlap() {
                            remaining_ships[current_ship_type_int] -= 1;
                            self.player_board.set_ship_type(
                                x,
                                y,
                                current_ship_type,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                            x = 0;
                            y = 0;
                            select_mode = true;
                        }
                    }
                    Ok(Key::Left) => {
                        if x > 0 {
                            self.player_board.set_ship_types(
                                x,
                                y,
                                &previous_ship_types,
                                horizontal,
                            );
                            x -= 1;
                            previous_ship_types = self.player_board.get_ship_types(
                                x,
                                y,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                            self.player_board.set_ship_type(
                                x,
                                y,
                                current_ship_type,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                        }
                    }
                    Ok(Key::Right) => {
                        let val = if horizontal {
                            current_ship_type.get_size()
                        } else {
                            1
                        };
                        if x < self.player_board.get_board_size() - val {
                            self.player_board.set_ship_types(
                                x,
                                y,
                                &previous_ship_types,
                                horizontal,
                            );
                            x += 1;
                            previous_ship_types = self.player_board.get_ship_types(
                                x,
                                y,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                            self.player_board.set_ship_type(
                                x,
                                y,
                                current_ship_type,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                        }
                    }
                    Ok(Key::Up) => {
                        if y > 0 {
                            self.player_board.set_ship_types(
                                x,
                                y,
                                &previous_ship_types,
                                horizontal,
                            );
                            y -= 1;
                            previous_ship_types = self.player_board.get_ship_types(
                                x,
                                y,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                            self.player_board.set_ship_type(
                                x,
                                y,
                                current_ship_type,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                        }
                    }
                    Ok(Key::Down) => {
                        let val = if horizontal {
                            1
                        } else {
                            current_ship_type.get_size()
                        };
                        if y < self.player_board.get_board_size() - val {
                            self.player_board.set_ship_types(
                                x,
                                y,
                                &previous_ship_types,
                                horizontal,
                            );
                            y += 1;
                            previous_ship_types = self.player_board.get_ship_types(
                                x,
                                y,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                            self.player_board.set_ship_type(
                                x,
                                y,
                                current_ship_type,
                                current_ship_type.get_size(),
                                horizontal,
                            );
                        }
                    }
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            }
            print!("{}[2J", 27 as char);
        }

        self.play_game();
    }

    fn play_game(&mut self) {
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        let mut previous_ship_type = self.guess_board.get_ship(x, y).unwrap().ship_type;

        self.guess_board
            .update_ship_selection(&mut previous_ship_type, x, y);

        let mut player_won = false;

        let mut finished = false;
        let g = Getch::new();

        while !finished {
            println!("Other Player's Board");
            self.guess_board.show();
            println!("Your Board");
            self.player_board.show();

            println!();

            println!("Change/Move Selection: Arrow Keys");
            println!("Confirm Selection: Enter Key");

            match g.getch() {
                Ok(Key::Left) => {
                    if x > 0 {
                        self.guess_board.get_ship(x, y).unwrap().ship_type = previous_ship_type;
                        x -= 1;
                        self.guess_board
                            .update_ship_selection(&mut previous_ship_type, x, y);
                    }
                }
                Ok(Key::Right) => {
                    if x < self.guess_board.get_board_size() - 1 {
                        self.guess_board.get_ship(x, y).unwrap().ship_type = previous_ship_type;
                        x += 1;
                        self.guess_board
                            .update_ship_selection(&mut previous_ship_type, x, y);
                    }
                }
                Ok(Key::Up) => {
                    if y > 0 {
                        self.guess_board.get_ship(x, y).unwrap().ship_type = previous_ship_type;
                        y -= 1;
                        self.guess_board
                            .update_ship_selection(&mut previous_ship_type, x, y);
                    }
                }
                Ok(Key::Down) => {
                    if y < self.guess_board.get_board_size() - 1 {
                        self.guess_board.get_ship(x, y).unwrap().ship_type = previous_ship_type;
                        y += 1;
                        self.guess_board
                            .update_ship_selection(&mut previous_ship_type, x, y);
                    }
                }
                Ok(Key::Char('\r')) => {
                    if previous_ship_type != ShipType::Shot {
                        self.ai_board.hit_ship(x, y);
                        let ship = self.guess_board.get_ship(x, y).unwrap();
                        ship.damaged = self.ai_board.get_ship(x, y).unwrap().damaged;
                        ship.ship_type = ShipType::Shot;
                        self.guess_board
                            .update_ship_selection(&mut previous_ship_type, x, y);
                        self.ai
                            .do_turn(&mut self.player_board, self.game_difficulty);
                        if self.ai_board.all_destroyed() || self.player_board.all_destroyed() {
                            finished = true;
                            player_won = self.ai_board.all_destroyed();
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }

            print!("{}[2J", 27 as char);
        }

        let mut color: Color = Color::Green;
        if !player_won {
            color = Color::Red;
        }
        println!(
            "{}",
            if player_won { "You Won!" } else { "AI Won!" }.color(color)
        );
        println!("Your Opponent's Board");
        self.ai_board.show();
        println!("Your Board");
        self.player_board.show();
    }
}
