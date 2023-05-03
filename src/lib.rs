#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, plot_num, plot_str, ColorCode, Color};
use pc_keyboard::{DecodedKey, KeyCode};

const PADDLE_HEIGHT: usize = 5;

pub enum GameState {
    MainMenu,
    SelectGameMode,
    DifficultySelect,
    Playing,
    GameOver,
}

pub enum GameMode {
    Footy,
    Hockey,
    Tennis,
}

pub enum Difficulty {
    Multiplayer,
    Easy, 
    Medium,
    Hard,
}

pub struct Game {
    player1: Player,
    player2: Player,
    tick_count: isize,
    ball: Ball,
    score1: u32,
    score2: u32,
    ball_speed: isize,
    game_mode: GameMode,
    game_state: GameState,
    difficulty: Difficulty,
}

impl Game {
    pub fn new() -> Self {
        Self {
            // player1: Player::new(2, BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2),
            player1: Player::new(2, BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2, 1),
            // player2: Player::new(BUFFER_WIDTH - 3, BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2),
            player2: Player::new(BUFFER_WIDTH - 3, BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2, 3),
            tick_count: 0,
            ball: Ball::new(BUFFER_WIDTH / 2, BUFFER_HEIGHT / 2, 1, 1),
            score1: 0,
            score2: 0,
            ball_speed: 2,
            game_mode: GameMode::Footy,
            game_state: GameState::MainMenu,
            difficulty: Difficulty::Multiplayer,
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
                    KeyCode::Enter => {
                        if let GameState::MainMenu = self.game_state {
                            self.game_state = GameState::SelectGameMode;
                        }
                    }
                    _ => {}
                }
            },
            DecodedKey::Unicode(key) => {
                match key {
                    'm' => {
                        if let GameState::GameOver = self.game_state {
                            self.restart_game();
                            self.game_state = GameState::MainMenu;
                        }
                    }
                    // 'p' => {
                    //     self.score1 = 6;
                    // }
                    'w' => {
                        self.player1.move_up();
                    }
                    's' => {
                        self.player1.move_down();
                    }
                    'r' => {
                        self.restart_game();
                    }
                    '\n' | '\r' => { // Handle 'Enter' key press in Unicode case as well
                        if let GameState::MainMenu = self.game_state {
                            self.game_state = GameState::SelectGameMode;
                        }
                    }
                    // ' ' => {
                    //     // PAUSE GAME ?
                    // }
                    'f' => {
                        if let GameState::SelectGameMode = self.game_state {
                            self.game_mode = GameMode::Footy;
                            self.game_state = GameState::DifficultySelect;
                        }
                    }
                    'h' => {
                        if let GameState::SelectGameMode = self.game_state {
                            self.game_mode = GameMode::Hockey;
                            self.game_state = GameState::DifficultySelect;
                        }
                    }
                    't' => {
                        if let GameState::SelectGameMode = self.game_state {
                            self.game_mode = GameMode::Tennis;
                            self.game_state = GameState::DifficultySelect;
                        }
                    }
                    '0' => {
                        if let GameState::DifficultySelect = self.game_state {
                            self.difficulty = Difficulty::Multiplayer;
                            self.game_state = GameState::Playing;
                        }
                    }
                    '1' => {
                        if let GameState::DifficultySelect = self.game_state {
                            self.difficulty = Difficulty::Easy;
                            self.game_state = GameState::Playing;
                        }
                    }
                    '2' => {
                        if let GameState::DifficultySelect = self.game_state {
                            self.difficulty = Difficulty::Medium;
                            self.game_state = GameState::Playing;
                        }
                    }
                    '3' => {
                        if let GameState::DifficultySelect = self.game_state {
                            self.difficulty = Difficulty::Hard;
                            self.game_state = GameState::Playing;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn tick(&mut self) {
        match self.game_state {
            GameState::MainMenu => {
                self.clear_screen();
                self.display_main_menu();
            }
            GameState::SelectGameMode => {
                self.clear_screen();
                self.display_game_mode_menu();
            }
            GameState::DifficultySelect => {
                self.clear_screen();
                self.display_difficulty_menu();
            }
            GameState::Playing => {
                match self.difficulty {
                    Difficulty::Multiplayer => {
                        self.cpu_move();
                    }
                    Difficulty::Easy => {
                        self.cpu_move();
                    }
                    Difficulty::Medium => {
                        self.cpu_move();
                    }
                    Difficulty::Hard => {
                        self.cpu_move();
                    }
                }
                self.tick_count += 1;
                let background_color = match self.game_mode {
                    GameMode::Footy => Color::Green,
                    GameMode::Hockey => Color::White,
                    GameMode::Tennis => Color::Blue,
                };
                self.clear_screen_playing(background_color);
                match self.game_mode {
                    GameMode::Footy => {
                        self.draw_soccer_field();
                    }
                    GameMode::Tennis => {
                        self.draw_tennis_court();
                    }
                    GameMode::Hockey => {
                        self.draw_hockey_rink();
                    }
                }
                self.render();
                let ball_color = match self.game_mode {
                    GameMode::Footy => Color::White,
                    GameMode::Hockey => Color::Black,
                    GameMode::Tennis => Color::Green,
                };
                self.ball.update_position(ball_color, background_color);
                self.handle_collisions();
        
                // Check for game over
                if self.score1 == 7 || self.score2 == 7 {
                    let winner = if self.score1 == 7 { 1 } else { 2 };
                    self.display_winner_message(winner);
                    self.game_state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                self.clear_screen();
                let winner = if self.score1 == 7 { 1 } else { 2 };
                self.display_winner_message(winner);
            }
        }
    }
    
    fn cpu_move(&mut self) {
        let ball_y = self.ball.y as isize;
        let player2_y = self.player2.y as isize;
        let player2_max_velocity = self.player2.max_velocity as isize;

        let velocity = match self.difficulty {
            Difficulty::Multiplayer => 0,
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
            _ => 0,
        };

        if velocity > 0 {
            let distance = ball_y - player2_y;
            let direction = distance.signum();
            let move_amount = (velocity * direction).min(player2_max_velocity);
            self.player2.y = (self.player2.y as isize + move_amount).max(0).min((BUFFER_HEIGHT - PADDLE_HEIGHT) as isize) as usize;
        }
    }
    
    fn display_main_menu(&self) {
        let game_name = "FOOTY-PONG";
        let game_name_x = (BUFFER_WIDTH / 2).saturating_sub(game_name.len() / 2);
        let game_name_y = BUFFER_HEIGHT / 2 - 2;
        let game_name_color = ColorCode::new(Color::Yellow, Color::Black);
        plot_str(game_name, game_name_x, game_name_y, game_name_color);

        let main_menu_message = "Press ENTER to start";
        let message_x = (BUFFER_WIDTH / 2).saturating_sub(main_menu_message.len() / 2);
        let message_y = BUFFER_HEIGHT / 2 + 1;
        let color = ColorCode::new(Color::White, Color::Black);
        plot_str(main_menu_message, message_x, message_y, color);
    }

    fn display_game_mode_menu(&self) {
        let game_mode_message = "Select Game Mode:";
        let game_mode_x = (BUFFER_WIDTH / 2).saturating_sub(game_mode_message.len() / 2);
        let game_mode_y = BUFFER_HEIGHT / 2 - 2;
        let color = ColorCode::new(Color::Yellow, Color::Black);
        plot_str(game_mode_message, game_mode_x, game_mode_y, color);

        let modes = [
            ("[F]ooty ", GameMode::Footy),
            ("[H]ockey", GameMode::Hockey),
            ("[T]ennis", GameMode::Tennis),
        ];

        for (i, (label, _)) in modes.iter().enumerate() {
            let x = (BUFFER_WIDTH / 2).saturating_sub(label.len() / 2);
            let y = game_mode_y + i as usize + 2;
            plot_str(label, x, y, ColorCode::new(Color::White, Color::Black));
        }
    }
    
    fn display_difficulty_menu(&self) {
        let gd_message = "Select Difficulty:";
        let gd_x = (BUFFER_WIDTH / 2).saturating_sub(gd_message.len() / 2);
        let gd_y = BUFFER_HEIGHT / 2 - 2;
        let color = ColorCode::new(Color::Yellow, Color::Black);
        plot_str(gd_message, gd_x, gd_y, color);

        let modes = [
            ("[0] Multiplayer", Difficulty::Multiplayer),
            ("[1] Easy       ", Difficulty::Easy),
            ("[2] Medium     ", Difficulty::Medium),
            ("[3] Hard       ", Difficulty::Hard),
        ];

        for (i, (label, _)) in modes.iter().enumerate() {
            let x = (BUFFER_WIDTH / 2).saturating_sub(label.len() / 2);
            let y = gd_y + i as usize + 2;
            plot_str(label, x, y, ColorCode::new(Color::White, Color::Black));
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
   
    pub fn draw_tennis_court(&self) {
        let court_color = ColorCode::new(Color::White, Color::Blue);
    
        // Draw the outer boundary
        for y in 0..BUFFER_HEIGHT {
            plot('|', 0, y, court_color);
            plot('|', BUFFER_WIDTH - 1, y, court_color);
        }
        for x in 0..BUFFER_WIDTH {
            plot('-', x, 0, court_color);
            plot('-', x, BUFFER_HEIGHT - 1, court_color);

            plot('-', x, 2, court_color);
            plot('-', x, BUFFER_HEIGHT - 3, court_color);
        }
    
        // Draw the horizontal line
        let horizontal_y = BUFFER_HEIGHT / 2;
        for x in BUFFER_WIDTH / 4..BUFFER_WIDTH * 3 / 4 {
            plot('-', x, horizontal_y, court_color);
        }
    
        // Draw the vertical lines
        let vertical_x1 = BUFFER_WIDTH / 4;
        // let vertical_x2 = BUFFER_WIDTH / 2;
        let vertical_x3 = BUFFER_WIDTH * 3 / 4;
    
        for y in 3..BUFFER_HEIGHT -3 {
            plot('|', vertical_x1, y, court_color);
            plot('|', vertical_x3, y, court_color);
            // plot('|', vertical_x2, y, court_color);
        }

        for y in 0..BUFFER_HEIGHT {
            plot('|', BUFFER_WIDTH / 2, y, court_color);
        }
    }
    
    pub fn draw_hockey_rink(&self) {
        let court_color = ColorCode::new(Color::Blue, Color::White);
        let red_lines = ColorCode::new(Color::Red, Color::White);
    
        // Draw the outer boundary
        for y in 0..BUFFER_HEIGHT {
            plot('|', 0, y, court_color);
            plot('|', BUFFER_WIDTH - 1, y, court_color);
        }
        for x in 0..BUFFER_WIDTH {
            plot('-', x, 0, court_color);
            plot('-', x, BUFFER_HEIGHT - 1, court_color);
        }
    
        // Draw the vertical lines
        let vertical_x1 = BUFFER_WIDTH / 4;
        let vertical_x2 = BUFFER_WIDTH / 2;
        let vertical_x3 = BUFFER_WIDTH * 3 / 4;
    
        for y in 0..BUFFER_HEIGHT {
            plot('|', vertical_x1, y, court_color);
            plot('|', vertical_x3, y, court_color);
            plot('|', vertical_x2, y, red_lines);
        }
    }
    
    fn clear_screen(&self) {
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                plot(' ', x, y, ColorCode::new(Color::Black, Color::Black));
            }
        }
    }

    
    fn clear_screen_playing(&self, background_color: Color) {
        
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                plot(' ', x, y, ColorCode::new(Color::Black, background_color));
            }
        }
    }

    fn render(&mut self) {
        let bg_color = match self.game_mode {
            GameMode::Footy => Color::Green,
            GameMode::Tennis => Color::Blue,
            GameMode::Hockey => Color::White,
        };
        let p1_color = match self.game_mode {
            GameMode::Footy => Color::Blue,
            GameMode::Tennis => Color::Yellow,
            GameMode::Hockey => Color::Blue,
        };
        self.player1.render(ColorCode::new(p1_color, bg_color));
        self.player2.render(ColorCode::new(Color::Red, bg_color));  
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
        let score_color = match self.game_mode {
            GameMode::Footy => Color::Green,
            GameMode::Hockey => Color::White,
            GameMode::Tennis => Color::Blue,
        };
        let p1_color = match self.game_mode {
            GameMode::Footy => Color::Blue,
            GameMode::Hockey => Color::Blue,
            GameMode::Tennis => Color::Yellow,
        };
        plot_num(self.score1 as isize, 30, 1, ColorCode::new(p1_color, score_color));
        plot_num(self.score2 as isize, 50, 1, ColorCode::new(Color::Red, score_color));
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
        // DEPENDS ON GAME MODE (TENNIS EXCEPTION)
        if self.score1 > self.score2 {
            winner_message = "Player 1 WINS!";
            wm_color = ColorCode::new(Color::Yellow, Color::Black);
        } else {
            winner_message = "Player 2 WINS!";
            wm_color = ColorCode::new(Color::Yellow, Color::Black);
        }
        let message_x = (BUFFER_WIDTH / 2).saturating_sub(winner_message.len() / 2);
        let message_y = (BUFFER_HEIGHT / 2) - 2;
        let color = ColorCode::new(Color::White, Color::Black);
        plot_str(&winner_message, message_x, message_y, wm_color);
    
        let main_menu_message = "[M]ain Menu";
        let main_menu_x = (BUFFER_WIDTH / 2).saturating_sub(main_menu_message.len() / 2);
        let main_menu_y = message_y + 2;
        plot_str(main_menu_message, main_menu_x, main_menu_y, color);

        let restart_message = "[R]estart";
        let restart_x = (BUFFER_WIDTH / 2).saturating_sub(restart_message.len() / 2);
        let restart_y = main_menu_y + 2;
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
    
    fn go_main_menu(&mut self) {
        if let GameState::GameOver = self.game_state {
            self.player1.y = BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2;
            self.player2.y = BUFFER_HEIGHT / 2 - PADDLE_HEIGHT / 2;
            self.ball.reset(BUFFER_WIDTH / 2, BUFFER_HEIGHT / 2, 1, 1);
            self.score1 = 0;
            self.score2 = 0;
            self.game_state = GameState::MainMenu;
        }
    }

}

// #[derive(Copy, Clone)]

pub struct Player {
    pub x: usize,
    pub y: usize,
    prev_y: usize,
    max_velocity: usize,
}

impl Player {

    pub fn new(x: usize, y: usize, max_velocity: usize) -> Self {
        Self { x, y, prev_y: y, max_velocity }
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
            plot(' ', self.x, self.prev_y + y_offset, ColorCode::new(color.foreground(), color.background()));
        }
        for y_offset in 0..PADDLE_HEIGHT {
            plot('#', self.x, self.y + y_offset, color);
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

    pub fn update_position(&mut self, ball_color: Color, bgcolor: Color) {
        plot(' ', self.prev_x, self.prev_y, ColorCode::new(Color::Black, bgcolor));
        
        self.x = (self.x as isize + self.x_velocity) as usize;
        self.y = (self.y as isize + self.y_velocity) as usize;
        plot('@', self.x, self.y, ColorCode::new(ball_color, bgcolor));

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

// Press ENTER to start

// -------------------------------

// Select Game Mode:
//
// [F]ooty (Classic mode)
// [H]ockey
// [T]ennis

// -------------------------------

// Select Difficulty:
//
// [0] Multiplayer 
// [1] Easy
// [2] Medium
// [3] Hard
