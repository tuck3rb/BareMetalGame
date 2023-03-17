#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_num}; //, plot_str};
use pc_keyboard::{DecodedKey, KeyCode};

const WALLS: &str = "################################################################################
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
################################################################################";


pub struct Game {
    player1: Player,
    player2: Player,
    tick_count: isize,
    ball: Ball,
    status: Status,
}

impl Game {
    pub fn new() -> Self {
        Self {player1: Player::new(10, 20), player2: Player::new(8, 12), tick_count: 0, ball: Ball::new(12,13,1,2), status: Status::Normal}
    }

    pub fn key(&mut self, key: DecodedKey) {
        panic!("{:?} {:?}", self.status, key);
        match self.status {
            Status::Normal => match key {
                DecodedKey::RawKey(key) => {
                    match key {
                        KeyCode::ArrowUp => {
                            self.player2.move_up();
                            panic!("Up: {} {}", self.player2.x, self.player2.y);
                        }
                        KeyCode::ArrowDown => {
                            self.player2.move_down();
                        }
                        KeyCode::W => {
                            self.player1.move_up();
                        }
                        KeyCode::S => {
                            self.player1.move_down();
                        }
                        // KeyCode::R => {
                        //     self.reset_game();
                        // }
                        _ => {}
                    }
                },
                DecodedKey::Unicode(_) => {}
            },
            Status::Over => {
            },
        }
    }

    // pub fn tick(&mut self) {
    //     self.tick_count += 1;
    //     plot('*', self.player1.x, self.player1.y, ColorCode::new(Color::Blue, Color::Black));
    //     plot('*', self.player2.x, self.player2.y, ColorCode::new(Color::Red, Color::Black));
    //     plot_num(self.tick_count, BUFFER_WIDTH / 2, 0, ColorCode::new(Color::LightGray, Color::Black));
    // }

    pub fn tick(&mut self) {
        // AM I NOT DRAWING THEM TO THE SCREEN WITH PLOT()???
        self.tick_count += 1;
        //panic!("{}, {} {}, {}", self.player1.x, self.player1.y, self.player2.x, self.player2.y);
        plot('*', self.player1.x, self.player1.y, ColorCode::new(Color::Blue, Color::Black));
        plot('*', self.player2.x, self.player2.y, ColorCode::new(Color::Red, Color::Black));
        
        // Update ball position
        self.ball.update_position();
        let ball_x = self.ball.x as isize;
        let ball_y = self.ball.y as isize;
        let ball_x_velocity = self.ball.x_velocity;
        let ball_y_velocity = self.ball.y_velocity;
        
        // Check for collision with top or bottom wall
        if ball_y <= 0 || ball_y >= (BUFFER_HEIGHT - 1) as isize {
            self.ball.change_direction(ball_x_velocity, -ball_y_velocity);
        }
        
        // Check for collision with player 1
        if ball_x == self.player1.x as isize && ball_y >= self.player1.y as isize && ball_y < (self.player1.y + 5) as isize {
            self.ball.change_direction(ball_x_velocity.abs(), (ball_y - self.player1.y as isize).signum());
        }
        
        // Check for collision with player 2
        if ball_x == self.player2.x as isize && ball_y >= self.player2.y as isize && ball_y < (self.player2.y + 5) as isize {
            self.ball.change_direction(-ball_x_velocity.abs(), (ball_y - self.player2.y as isize).signum());
        }
        
        // Plot ball
        plot('O', ball_x as usize, ball_y as usize, ColorCode::new(Color::White, Color::Black));
        
        plot_num(self.tick_count, BUFFER_WIDTH / 2, 0, ColorCode::new(Color::LightGray, Color::Black));
    }
    
}

#[derive(Debug)]
pub enum Status {
    Normal,
    Over,
}

#[derive(Copy, Clone)]

pub struct Player {
    pub x: usize,
    pub y: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < BUFFER_HEIGHT - 1 {
            self.y += 1;
        }
    }
}

pub struct Ball {
    pub x: usize,
    pub y: usize,
    pub x_velocity: isize,
    pub y_velocity: isize,
}

impl Ball {
    pub fn new(x: usize, y: usize, x_velocity: isize, y_velocity: isize) -> Self {
        Self { x, y, x_velocity, y_velocity }
    }

    pub fn update_position(&mut self) {
        self.x = (self.x as isize + self.x_velocity) as usize;
        self.y = (self.y as isize + self.y_velocity) as usize;
    }

    pub fn change_direction(&mut self, x_velocity: isize, y_velocity: isize) {
        self.x_velocity = x_velocity;
        self.y_velocity = y_velocity;
    }
}
