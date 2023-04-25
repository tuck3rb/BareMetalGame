#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, plot_num, plot_str, ColorCode, Color};
use pc_keyboard::{DecodedKey, KeyCode};

const PADDLE_HEIGHT: usize = 5;

pub enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    player1: Player,
    player2: Player,
    tick_count: isize,
    ball: Ball,
    score1: u32,
    score2: u32,
    ball_speed: isize,
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player1: Player::new(2, BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2),
            player2: Player::new(BUFFER_WIDTH - 3, BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2),
            tick_count: 0,
            ball: Ball::new(BUFFER_WIDTH / 2, BUFFER_HEIGHT / 2, 1, 1),
            score1: 0,
            score2: 0,
            ball_speed: 2,
            game_state: GameState::Playing,
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(key) => {
                match key {
                    KeyCode::ArrowUp => {
                        self.player2.move_up();
                    }
                    KeyCode::ArrowDown => {
                        self.player2.move_down();
                    }
                    _ => {}
                }
            },
            DecodedKey::Unicode(key) => {
                match key {
                    'w' => {
                        self.player1.move_up();
                    }
                    's' => {
                        self.player1.move_down();
                    }
                    'r' => {
                        self.restart_game();
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.clear_screen();
        self.draw_soccer_field();
    
        match self.game_state {
            GameState::Playing => {
                self.tick_count += 1;
                self.render();
                self.ball.update_position();
                self.handle_collisions();
    
                // Check for game over
                if self.score1 == 7 || self.score2 == 7 {
                    let winner = if self.score1 == 7 { 1 } else { 2 };
                    self.display_winner_message(winner);
                    self.game_state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                let winner = if self.score1 == 7 { 1 } else { 2 };
                self.display_winner_message(winner);
            }
        }
    }
    
    

    pub fn draw_soccer_field(&self) {
        let field_color = ColorCode::new(Color::White, Color::Green);

        // Draw the outer boundary
        for y in 0..BUFFER_HEIGHT {
            plot('|', 0, y, field_color);
            plot('|', BUFFER_WIDTH - 1, y, field_color);
        }
        for x in 0..BUFFER_WIDTH {
            plot('-', x, 0, field_color);
            plot('-', x, BUFFER_HEIGHT - 1, field_color);
        }

        // Draw the halfway line
        let halfway_x = BUFFER_WIDTH / 2;
        for y in 0..BUFFER_HEIGHT {
            plot('|', halfway_x, y, field_color);
        }

        // Draw the penalty boxes
        let box_width = BUFFER_WIDTH / 6; // Decrease the width
        let box_height = (3 * BUFFER_HEIGHT) / 4; // Increase the height
        let top_y = (BUFFER_HEIGHT - box_height) / 2;
        let bottom_y = top_y + box_height;

        // Left penalty box
        for y in top_y..=bottom_y {
            plot('|', box_width, y, field_color);
        }
        for x in 0..=box_width {
            plot('-', x, top_y, field_color);
            plot('-', x, bottom_y, field_color);
        }

        // Right penalty box
        for y in top_y..=bottom_y {
            plot('|', BUFFER_WIDTH - box_width - 1, y, field_color);
        }
        for x in (BUFFER_WIDTH - box_width)..BUFFER_WIDTH {
            plot('-', x, top_y, field_color);
            plot('-', x, bottom_y, field_color);
        }

        // Center circle
        // TBD

        // Penalty arcs
        // TBD

    }
    
    fn clear_screen(&self) {
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                plot(' ', x, y, ColorCode::new(Color::Black, Color::Green));
            }
        }
    }

    fn render(&mut self) {
        self.player1.render(ColorCode::new(Color::Blue, Color::Black));
        self.player2.render(ColorCode::new(Color::Red, Color::Black));  
        self.display_score();  
    }
    
    fn handle_collisions(&mut self) {
        let ball_x = self.ball.x as isize;
        let ball_y = self.ball.y as isize;
        let ball_x_velocity = self.ball.x_velocity;
        let ball_y_velocity = self.ball.y_velocity;

        // Check for collision with top or bottom wall
        if ball_y <= 0 || ball_y >= (BUFFER_HEIGHT - 1) as isize {
            self.ball.change_direction(ball_x_velocity, -ball_y_velocity);
        }

        // Check for collision with player 1
        if ball_x == self.player1.x as isize && ball_y >= self.player1.y as isize && ball_y < (self.player1.y + PADDLE_HEIGHT) as isize {
            self.ball.change_direction(ball_x_velocity.abs(), (ball_y - self.player1.y as isize).signum());
        }

        // Check for collision with player 2
        if ball_x == self.player2.x as isize && ball_y >= self.player2.y as isize && ball_y < (self.player2.y + PADDLE_HEIGHT) as isize {
            self.ball.change_direction(-ball_x_velocity.abs(), (ball_y - self.player2.y as isize).signum());
        }
        
        // Check for a point scored by player 1
        if ball_x >= (BUFFER_WIDTH - 1) as isize {
            self.score1 += 1;
            if self.score1 == 7 {
                self.game_state = GameState::GameOver;
            } else {
                self.ball.reset(BUFFER_WIDTH / 2, BUFFER_HEIGHT / 2, -1, 1);
            }
        }

        // Check for a point scored by player 2
        if ball_x <= 0 {
            self.score2 += 1;
            if self.score2 == 7 {
                self.game_state = GameState::GameOver;
            } else {
                self.ball.reset(BUFFER_WIDTH / 2, BUFFER_HEIGHT / 2, 1, 1);
            }
        }

    }

    fn display_score(&mut self) {
        plot_num(self.score1 as isize, 30, 1, ColorCode::new(Color::Blue, Color::Green));
        plot_num(self.score2 as isize, 50, 1, ColorCode::new(Color::Red, Color::Green));
    }

    fn check_for_winner(&self) -> u8 {
        if self.score1 >= 7 {
            1
        } else if self.score2 >= 7 {
            2
        } else {
            0
        }
    }

    fn display_winner_message(&self, winner: u8) {
        let mut winner_message;
        let mut wm_color;
        if self.score1 > self.score2 {
            winner_message = "Blue WINS!";
            wm_color = ColorCode::new(Color::Blue, Color::Green);
        } else {
            winner_message = "Red WINS!";
            wm_color = ColorCode::new(Color::Red, Color::Green);
        }
        let message_x = (BUFFER_WIDTH / 2).saturating_sub(winner_message.len() / 2);
        let message_y = BUFFER_HEIGHT / 2;
        let color = ColorCode::new(Color::White, Color::Green);
        plot_str(&winner_message, message_x, message_y, wm_color);
    
        let restart_message = "Press R to restart";
        let restart_x = (BUFFER_WIDTH / 2).saturating_sub(restart_message.len() / 2);
        let restart_y = message_y + 2;
        plot_str(restart_message, restart_x, restart_y, color);
    }
    
    

    
    fn restart_game(&mut self) {
        if let GameState::GameOver = self.game_state {
            self.player1.y = BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2;
            self.player2.y = BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2;
            self.ball.reset(BUFFER_WIDTH / 2, BUFFER_HEIGHT / 2, 1, 1);
            self.score1 = 0;
            self.score2 = 0;
            self.game_state = GameState::Playing;
        }
    }
    

}

// #[derive(Copy, Clone)]

pub struct Player {
    pub x: usize,
    pub y: usize,
    prev_y: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, prev_y: y}
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < BUFFER_HEIGHT - PADDLE_HEIGHT {
            self.y += 1;
        }
    }

    pub fn render(&mut self, color: ColorCode) {
        for y_offset in 0..PADDLE_HEIGHT {
            plot(' ', self.x, self.prev_y + y_offset, ColorCode::new(color.foreground(), Color::Green));        
        }
        for y_offset in 0..PADDLE_HEIGHT {
            plot('|', self.x, self.y + y_offset, ColorCode::new(color.foreground(), Color::Green));
        }
        self.prev_y = self.y;
    }

}

pub struct Ball {
    pub x: usize,
    pub y: usize,
    pub x_velocity: isize,
    pub y_velocity: isize,
    prev_x: usize,
    prev_y: usize,
}

impl Ball {
    pub fn new(x: usize, y: usize, x_velocity: isize, y_velocity: isize) -> Self {
        Self { x, y, x_velocity, y_velocity, prev_x: x, prev_y: y}
    }

    pub fn update_position(&mut self) {
        plot(' ', self.prev_x, self.prev_y, ColorCode::new(Color::Black, Color::Green));
        
        self.x = (self.x as isize + self.x_velocity) as usize;
        self.y = (self.y as isize + self.y_velocity) as usize;
        plot('@', self.x, self.y, ColorCode::new(Color::White, Color::Green));

        self.prev_x = self.x;
        self.prev_y = self.y;
    }

    pub fn change_direction(&mut self, x_velocity: isize, y_velocity: isize) {
        self.x_velocity = x_velocity;
        self.y_velocity = y_velocity;
    }

    pub fn reset(&mut self, x: usize, y: usize, x_velocity: isize, y_velocity: isize) {
        self.x = x;
        self.y = y;
        self.x_velocity = x_velocity;
        self.y_velocity = y_velocity;
    }

}
