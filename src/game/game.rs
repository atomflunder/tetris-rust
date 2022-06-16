use std::path;

use ggez::{
    event,
    graphics::{self, Color, Text, TextFragment},
    timer, GameError, GameResult,
};
use glam::Vec2;
use thousands::Separable;

use super::{board::Board, config::Config};
use super::{config::load_config, pieces::Piece};

#[derive(Clone)]
pub struct MainGame {
    pub board: Board,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub piece_bag: Vec<Piece>,
    pub score: u128,
    pub level: u128,
    pub lines_cleared: u128,
    // You start with no held piece, so this needs to be an option.
    // This will always be populated after you hold one piece though.
    pub held_piece: Option<Piece>,
    // You can only swap your hold piece once per turn.
    pub can_swap: bool,
    // Keeps track of how many pieces have been spawned.
    pub piece_count: Vec<u128>,
    // Keeps track of how many lines you clear at once.
    pub clear_count: Vec<u128>,
    // You get 1 score for how far you drop a piece down, this keeps track of that feature.
    // If you hold down from a height of 16 you will get 16 points.
    pub down_presses: u128,
    pub paused: bool,
    pub game_over: bool,
    pub config: Config,
}

impl MainGame {
    fn new() -> GameResult<MainGame> {
        let config = load_config();

        let mut piece_bag = Piece::get_new_piece_bag(config.clone(), true);

        let piece = Piece::get_random_piece(&mut piece_bag, config.clone(), true);
        let next_piece = Piece::get_random_piece(&mut piece_bag, config.clone(), true);

        let mut b = MainGame {
            board: Board::new(),
            current_piece: piece.clone(),
            next_piece: next_piece.clone(),
            piece_bag: piece_bag,
            score: 0,
            level: 1,
            lines_cleared: 0,
            held_piece: None,
            can_swap: true,
            piece_count: vec![0; 7],
            clear_count: vec![0; 4],
            down_presses: 0,
            paused: false,
            game_over: false,
            config: config,
        };

        Piece::spawn_piece(piece, &mut b, false);

        Ok(b)
    }

    /// Deletes "full" lines on the game board.
    pub fn erase_lines(
        ctx: &mut ggez::Context,
        board: &mut Board,
        score: &mut u128,
        level: u128,
        clear_count: &mut Vec<u128>,
    ) -> u8 {
        let mut erase_count: u8 = 0;

        for line in board.board {
            if line == ['#'; 10] {
                erase_count += 1;
            }
        }

        // First deletes the full lines and then inserts new lines at the top.
        let mut temp_vec_board = board.board.to_vec();
        let mut temp_vec_color = board.color.to_vec();

        temp_vec_board.retain(|l| l != &['#'; 10]);
        temp_vec_color.retain(|l| l.contains(&(255, 255, 255)));

        for _ in 0..erase_count {
            temp_vec_board.insert(0, [' '; 10]);
            temp_vec_color.insert(0, [(255, 255, 255); 10]);
        }

        if erase_count != 0 {
            let red_line = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, 500.0, 50.0),
                Color::new(
                    // Basically the more lines erased, the more red the color will be.
                    // A tetris being full red.
                    1.0,
                    0.5 - (erase_count as f32 / 8.0),
                    0.5 - (erase_count as f32 / 8.0),
                    1.0,
                ),
            )
            .unwrap();

            for (x, line) in board.board.iter().enumerate() {
                if !line.contains(&' ') {
                    graphics::draw(
                        ctx,
                        &red_line,
                        graphics::DrawParam::default().dest([0.0, x as f32 * 50.0]),
                    )
                    .unwrap();
                }
            }

            graphics::present(ctx).unwrap();

            // Sleeping for a short bit to make it more "impactful" I guess,
            // also it gives the player a small bit of breathing room.
            let dur = std::time::Duration::from_millis(200);
            ggez::timer::sleep(dur);
        }

        // The scores are from the NES Tetris Game, seemed pretty good.
        match erase_count {
            1 => {
                *score += 40 * level;
                clear_count[0] += 1;
            }
            2 => {
                *score += 100 * level;
                clear_count[1] += 1;
            }
            3 => {
                *score += 300 * level;
                clear_count[2] += 1;
            }
            4 => {
                *score += 1200 * level;
                clear_count[3] += 1;
            }
            _ => (),
        }

        board.board = temp_vec_board.try_into().unwrap_or(board.board);
        board.color = temp_vec_color.try_into().unwrap_or(board.color);

        erase_count
    }

    /// Resets the game.
    pub fn reset_game(&mut self) {
        let config = load_config();

        let mut piece_bag = Piece::get_new_piece_bag(config.clone(), true);

        let piece = Piece::get_random_piece(&mut piece_bag, config.clone(), true);
        let next_piece = Piece::get_random_piece(&mut piece_bag, config.clone(), true);

        self.board = Board::new();
        self.current_piece = piece.clone();
        self.next_piece = next_piece.clone();
        self.piece_bag = piece_bag;
        self.score = 0;
        self.level = 1;
        self.lines_cleared = 0;
        self.held_piece = None;
        self.can_swap = true;
        self.piece_count = vec![0; 7];
        self.clear_count = vec![0; 4];
        self.down_presses = 0;
        self.paused = false;
        self.game_over = false;
        self.config = config;
    }
}

impl event::EventHandler<GameError> for MainGame {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while timer::check_update_time(ctx, (self.level as f32 / 5.0).ceil() as u32)
            && self.paused == false
        {
            Piece::move_piece_down(self, false, ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        graphics::clear(ctx, Color::BLACK);

        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 49.0, 49.0),
            Color::WHITE,
        )?;

        let shadow_square_outline = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(0.0, 0.0, 49.0, 49.0),
            self.current_piece.color.into(),
        )?;

        let shadow_square_fill = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 49.0, 49.0),
            Color::new(
                self.current_piece.color.0 as f32,
                self.current_piece.color.1 as f32,
                self.current_piece.color.2 as f32,
                0.1,
            ),
        )?;

        let mini_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 11.0, 11.0),
            Color::WHITE,
        )?;

        let piece_windows = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 280.0, 180.0),
            Color::BLACK,
        )?;

        let count_window = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(510.0, 650.0, 280.0, 340.0),
            Color::BLACK,
        )?;

        let font = graphics::Font::new(ctx, "/fonts/PressStart2P-Regular.ttf")?;

        let level_text = Text::new(
            // The font looks better in all caps, in my opinion.
            TextFragment::new(format!("LEVEL: {}", self.level.separate_with_commas()))
                .font(font)
                .scale(28.0),
        );

        let lines_text = Text::new(
            TextFragment::new(format!(
                "LINES: \n{}",
                self.lines_cleared.separate_with_commas()
            ))
            .font(font)
            .scale(28.0),
        );

        let score_text = Text::new(
            TextFragment::new(format!("SCORE: \n{}", self.score.separate_with_commas()))
                .font(font)
                .scale(28.0),
        );

        let held_text = Text::new(TextFragment::new("HOLD:").font(font).scale(28.0));
        let next_text = Text::new(TextFragment::new("NEXT:").font(font).scale(28.0));

        let count_text = Text::new(
            TextFragment::new(format!(
                "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
                self.piece_count[0].separate_with_commas(),
                self.piece_count[1].separate_with_commas(),
                self.piece_count[2].separate_with_commas(),
                self.piece_count[3].separate_with_commas(),
                self.piece_count[4].separate_with_commas(),
                self.piece_count[5].separate_with_commas(),
                self.piece_count[6].separate_with_commas(),
            ))
            .font(font)
            .scale(24.0),
        );

        let line_count_text = Text::new(
            TextFragment::new(format!(
                "SINGLE: \n{}\n\nDOUBLE: \n{}\n\nTRIPLE: \n{}\n\nTETRIS: \n{}",
                self.clear_count[0].separate_with_commas(),
                self.clear_count[1].separate_with_commas(),
                self.clear_count[2].separate_with_commas(),
                self.clear_count[3].separate_with_commas()
            ))
            .font(font)
            .scale(18.0),
        );

        let game_over_text = Text::new(TextFragment::new("GAME OVER!").font(font).scale(50.0));
        let restart_text = Text::new(
            TextFragment::new("HOLD ENTER TO RESTART.")
                .font(font)
                .scale(21.0),
        );

        // The shadow coordinates are the coordinates of the piece if it were dropped.
        // So you can see where the piece will end up.
        let shadow_coordinates = Piece::get_shadow_piece(self, ctx);

        let mut temp_piece_pos: Vec<(usize, usize)> = vec![];

        for block in self.current_piece.orientations[self.current_piece.rotations].clone() {
            let temp_block_0 = block.0 + self.current_piece.offset.0;
            let temp_block_1 = block.1 + self.current_piece.offset.1;
            temp_piece_pos.push((temp_block_0, temp_block_1));
        }

        let mut background_color = (77, 77, 204).into();

        let first_five_lines = self.board.board[0..5].to_vec();

        'outer: for (y, line) in first_five_lines.iter().enumerate() {
            for (x, block) in line.iter().enumerate() {
                if !temp_piece_pos.contains(&(y, x)) && block == &'#' {
                    background_color = (204, 77, 77).into();
                    break 'outer;
                }
            }
        }

        let menu_background = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(500.0, 0.0, 300.0, 1000.0),
            background_color,
        )?;

        // We stop drawing the board if you reach game over.
        if self.game_over == false {
            for (y, line) in self.board.board.iter().enumerate() {
                for (x, block) in line.iter().enumerate() {
                    if shadow_coordinates.contains(&(y, x)) {
                        graphics::draw(
                            ctx,
                            &shadow_square_outline,
                            graphics::DrawParam::default()
                                .dest(Vec2::new((x as f32) * 50.0, (y as f32) * 50.0)),
                        )?;
                        graphics::draw(
                            ctx,
                            &shadow_square_fill,
                            graphics::DrawParam::default()
                                .dest(Vec2::new((x as f32) * 50.0, (y as f32) * 50.0)),
                        )?;
                    }

                    if block == &'#' {
                        let mut block_color = self.board.color[y][x];

                        if self.config.colored_board == false && !temp_piece_pos.contains(&(y, x)) {
                            block_color = (255, 255, 255);
                        }

                        graphics::draw(
                            ctx,
                            &square,
                            graphics::DrawParam::default()
                                .dest(Vec2::new((x as f32) * 50.0, (y as f32) * 50.0))
                                .color(block_color.into()),
                        )?;
                    }
                }
            }
        } else {
            graphics::draw(
                ctx,
                &game_over_text,
                graphics::DrawParam::default().dest([5.0, 200.0]),
            )?;
            graphics::draw(
                ctx,
                &restart_text,
                graphics::DrawParam::default().dest([15.0, 255.0]),
            )?;
        }

        graphics::draw(ctx, &menu_background, graphics::DrawParam::default())?;

        graphics::draw(
            ctx,
            &level_text,
            graphics::DrawParam::default().dest([510.0, 10.0]),
        )?;

        graphics::draw(
            ctx,
            &lines_text,
            graphics::DrawParam::default().dest([510.0, 55.0]),
        )?;

        graphics::draw(
            ctx,
            &score_text,
            graphics::DrawParam::default().dest([510.0, 155.0]),
        )?;

        graphics::draw(
            ctx,
            &piece_windows,
            graphics::DrawParam::default().dest([510.0, 250.0]),
        )?;

        graphics::draw(
            ctx,
            &next_text,
            graphics::DrawParam::default().dest([510.0, 250.0]),
        )?;

        graphics::draw(
            ctx,
            &piece_windows,
            graphics::DrawParam::default().dest([510.0, 450.0]),
        )?;

        graphics::draw(
            ctx,
            &held_text,
            graphics::DrawParam::default().dest([510.0, 450.0]),
        )?;

        graphics::draw(ctx, &count_window, graphics::DrawParam::default())?;

        graphics::draw(
            ctx,
            &count_text,
            graphics::DrawParam::default().dest([580.0, 655.0]),
        )?;

        // We draw miniature versions of the pieces in the menu, for the piece counts.
        // There is probably a better way to do this.
        let i_piece = Piece::get_i_piece();
        let l_piece = Piece::get_l_piece();
        let j_piece = Piece::get_j_piece();
        let s_piece = Piece::get_s_piece();
        let z_piece = Piece::get_z_piece();
        let o_piece = Piece::get_o_piece();
        let t_piece = Piece::get_t_piece();

        let all_pieces = [
            i_piece, l_piece, j_piece, s_piece, z_piece, o_piece, t_piece,
        ];

        for (x, piece) in all_pieces.iter().enumerate() {
            for block in &piece.orientations[0] {
                graphics::draw(
                    ctx,
                    &mini_square,
                    graphics::DrawParam::default()
                        .dest([
                            515.0 + (block.1 as f32 * 12.0),
                            655.0 + (block.0 as f32 * 12.0) + (x as f32 * 48.0),
                        ])
                        .color(piece.color.into()),
                )?;
            }
        }

        for block in &self.next_piece.orientations[0] {
            graphics::draw(
                ctx,
                &square,
                graphics::DrawParam::default()
                    .dest([
                        520.0 + (block.1 as f32 * 50.0),
                        300.0 + (block.0 as f32 * 50.0),
                    ])
                    .color(self.next_piece.color.into()),
            )?;
        }

        graphics::draw(
            ctx,
            &line_count_text,
            graphics::DrawParam::default().dest([665.0, 655.0]),
        )?;

        if !self.held_piece.is_none() {
            let held_color: Color;

            // If you cant switch the held piece, we will color it grey,
            // otherwise it will be the actual piece color.
            if self.can_swap == true {
                held_color = self.held_piece.clone().unwrap().color.into()
            } else {
                held_color = (60, 60, 60).into()
            }

            for block in &self.held_piece.clone().unwrap().orientations[0] {
                graphics::draw(
                    ctx,
                    &square,
                    graphics::DrawParam::default()
                        .dest([
                            520.0 + (block.1 as f32 * 50.0),
                            500.0 + (block.0 as f32 * 50.0),
                        ])
                        .color(held_color),
                )?;
            }
        }

        if self.paused == true {
            let paused_text = Text::new(TextFragment::new("PAUSED").font(font).scale(75.0));

            graphics::draw(
                ctx,
                &paused_text,
                graphics::DrawParam::default().dest([30.0, 200.0]),
            )?;
        }

        graphics::present(ctx)?;

        graphics::clear_font_cache(ctx);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        repeat: bool,
    ) {
        // If the game is paused, we dont listen to any keystrokes except for Escape.
        if self.paused == false {
            match keycode {
                event::KeyCode::Left => {
                    Piece::move_piece_left(&mut self.current_piece, &mut self.board);
                }
                event::KeyCode::Right => {
                    Piece::move_piece_right(&mut self.current_piece, &mut self.board);
                }
                event::KeyCode::Down => {
                    self.down_presses += 1;
                    Piece::move_piece_down(self, false, ctx);
                }
                event::KeyCode::Up => {
                    Piece::drop_piece_down(self, false, ctx);
                }
                event::KeyCode::Space => {
                    Piece::rotate_piece(&mut self.current_piece, &mut self.board, true);
                }
                event::KeyCode::LAlt => {
                    Piece::rotate_piece(&mut self.current_piece, &mut self.board, false);
                }
                event::KeyCode::Numpad0 => {
                    Piece::hold_piece(self);
                }
                event::KeyCode::Return => {
                    if self.game_over == true && repeat == true {
                        self.reset_game();
                    }
                }
                event::KeyCode::Escape => {
                    // No real reason to pause on the game over screen.
                    if self.game_over == false {
                        self.paused = true;
                    }
                }
                _ => (),
            }
        } else {
            match keycode {
                event::KeyCode::Escape => {
                    self.paused = false;
                }
                _ => (),
            }
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
    ) {
        // Resetting the down presses when you release the down key.
        match keycode {
            event::KeyCode::Down => {
                self.down_presses = 0;
            }
            _ => (),
        }
    }
}

pub fn run() -> GameResult {
    let window = ggez::conf::WindowMode::default().dimensions(800.0, 1000.0);

    let mut asset_path = path::PathBuf::from("./");
    asset_path.push("resources");

    let window_setup = ggez::conf::WindowSetup::default()
        .title("Tetris!")
        .srgb(true)
        .icon("/icons/icon.png");

    let (ctx, event_loop) = ggez::ContextBuilder::new("Tetris", "atomflunder")
        .window_setup(window_setup)
        .window_mode(window)
        .add_resource_path(asset_path)
        .build()?;

    let board = MainGame::new()?;

    event::run(ctx, event_loop, board)
}
