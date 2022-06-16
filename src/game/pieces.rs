use std::fmt;

use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::board::Board;
use super::config::Config;
use super::game::MainGame;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    I,
    L,
    J,
    S,
    Z,
    O,
    T,
}
#[derive(Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: (u8, u8, u8),
    pub offset: (usize, usize),
    pub rotations: usize,
    pub orientations: Vec<Vec<(usize, usize)>>,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.piece_type)
    }
}

impl Piece {
    /// We get a new random piece depending on the piece RNG selected.
    pub fn get_random_piece(piece_bag: &mut Vec<Piece>, config: Config, first_bag: bool) -> Piece {
        if config.modern_piece_rng == false {
            return Self::get_random_piece_classic();
        }

        Self::get_random_piece_modern(piece_bag, config, first_bag)
    }

    /// Gets a completely random piece, the "oldschool" Tetris Piece Algorithm.
    pub fn get_random_piece_classic() -> Piece {
        let random_pieces = [
            Self::get_i_piece(),
            Self::get_l_piece(),
            Self::get_j_piece(),
            Self::get_s_piece(),
            Self::get_z_piece(),
            Self::get_o_piece(),
            Self::get_t_piece(),
        ];

        random_pieces
            .choose(&mut thread_rng())
            .unwrap_or(&Self::get_i_piece())
            .clone()
    }

    /// Gets a shuffled sequence of 7 Pieces, the modern Tetris Piece Algorithm.
    pub fn get_random_piece_modern(
        piece_bag: &mut Vec<Piece>,
        config: Config,
        first_bag: bool,
    ) -> Piece {
        if piece_bag.is_empty() {
            *piece_bag = Self::get_new_piece_bag(config, first_bag);
        }

        piece_bag.pop().unwrap()
    }

    // These are its own functions because we need the individual pieces elsewhere too.
    pub fn get_i_piece() -> Piece {
        Piece {
            piece_type: PieceType::I,
            color: (0, 255, 255),
            offset: (0, 3),
            rotations: 0,
            orientations: vec![
                vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            ],
        }
    }

    pub fn get_l_piece() -> Piece {
        Piece {
            piece_type: PieceType::L,
            color: (255, 127, 0),
            offset: (0, 3),
            rotations: 0,
            orientations: vec![
                vec![(1, 1), (1, 0), (1, 2), (0, 2)],
                vec![(0, 0), (1, 0), (2, 0), (2, 1)],
                vec![(0, 0), (0, 1), (0, 2), (1, 0)],
                vec![(0, 0), (0, 1), (1, 1), (2, 1)],
            ],
        }
    }

    pub fn get_j_piece() -> Piece {
        Piece {
            piece_type: PieceType::J,
            color: (0, 0, 255),
            offset: (0, 3),
            rotations: 0,
            orientations: vec![
                vec![(0, 0), (1, 0), (1, 1), (1, 2)],
                vec![(0, 0), (0, 1), (1, 0), (2, 0)],
                vec![(0, 0), (0, 1), (0, 2), (1, 2)],
                vec![(0, 1), (1, 1), (2, 1), (2, 0)],
            ],
        }
    }

    pub fn get_s_piece() -> Piece {
        Piece {
            piece_type: PieceType::S,
            color: (0, 255, 0),
            offset: (0, 3),
            rotations: 0,
            orientations: vec![
                vec![(1, 0), (1, 1), (0, 1), (0, 2)],
                vec![(0, 0), (1, 0), (1, 1), (2, 1)],
                vec![(1, 0), (1, 1), (0, 1), (0, 2)],
                vec![(0, 0), (1, 0), (1, 1), (2, 1)],
            ],
        }
    }

    pub fn get_z_piece() -> Piece {
        Piece {
            piece_type: PieceType::Z,
            color: (255, 0, 0),
            offset: (0, 3),
            rotations: 0,
            orientations: vec![
                vec![(0, 0), (0, 1), (1, 1), (1, 2)],
                vec![(1, 0), (2, 0), (0, 1), (1, 1)],
                vec![(0, 0), (0, 1), (1, 1), (1, 2)],
                vec![(1, 0), (2, 0), (0, 1), (1, 1)],
            ],
        }
    }

    pub fn get_o_piece() -> Piece {
        Piece {
            piece_type: PieceType::O,
            color: (255, 255, 0),
            offset: (0, 3),
            rotations: 0,
            orientations: vec![
                vec![(0, 0), (0, 1), (1, 0), (1, 1)],
                vec![(0, 0), (0, 1), (1, 0), (1, 1)],
                vec![(0, 0), (0, 1), (1, 0), (1, 1)],
                vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            ],
        }
    }

    pub fn get_t_piece() -> Piece {
        Piece {
            piece_type: PieceType::T,
            color: (128, 0, 128),
            offset: (0, 3),
            rotations: 0,
            orientations: vec![
                vec![(1, 0), (1, 1), (0, 1), (1, 2)],
                vec![(0, 0), (1, 0), (2, 0), (1, 1)],
                vec![(0, 0), (0, 1), (0, 2), (1, 1)],
                vec![(1, 0), (0, 1), (1, 1), (2, 1)],
            ],
        }
    }

    /// Gets a new "bag" of pieces, each Piece X times, shuffled.
    pub fn get_new_piece_bag(config: Config, first_bag: bool) -> Vec<Piece> {
        let mut random_pieces: Vec<Piece> = Vec::new();
        let mut bags = config.bag_amount;

        if bags < 1 {
            bags = 1;
        }

        for _ in 0..bags {
            random_pieces.push(Self::get_i_piece());
            random_pieces.push(Self::get_l_piece());
            random_pieces.push(Self::get_j_piece());
            random_pieces.push(Self::get_s_piece());
            random_pieces.push(Self::get_z_piece());
            random_pieces.push(Self::get_o_piece());
            random_pieces.push(Self::get_t_piece());
        }

        random_pieces.shuffle(&mut thread_rng());

        // If the first piece no overhang setting is set to true,
        // we will prevent Z,S & Os spawning as the first piece to, well, prevent overhangs.
        if first_bag == true && config.first_piece_no_overhang == true {
            while [PieceType::Z, PieceType::S, PieceType::O]
                .contains(&random_pieces.last().unwrap().piece_type)
            {
                random_pieces.shuffle(&mut thread_rng());
            }
        }

        random_pieces.to_vec()
    }

    /// Spawning a new piece on the board.
    pub fn spawn_piece(piece: Piece, game: &mut MainGame, spawn_held: bool) {
        if game.game_over == true {
            return;
        }

        let piece_blocks = piece.orientations[0].clone();

        if spawn_held == false {
            match piece.piece_type {
                PieceType::I => game.piece_count[0] += 1,
                PieceType::L => game.piece_count[1] += 1,
                PieceType::J => game.piece_count[2] += 1,
                PieceType::S => game.piece_count[3] += 1,
                PieceType::Z => game.piece_count[4] += 1,
                PieceType::O => game.piece_count[5] += 1,
                PieceType::T => game.piece_count[6] += 1,
            }
        }

        for block in piece_blocks {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            if game.board.board[temp_block_0][temp_block_1] == ' ' {
                game.board.board[temp_block_0][temp_block_1] = '#';
                game.board.color[temp_block_0][temp_block_1] = piece.color;
            } else if spawn_held == false {
                // If there is no room to spawn a new piece we set the game over flag to true.
                game.game_over = true;
            }
        }
    }

    /// Holds a piece and spawns the old piece held, if available.
    pub fn hold_piece(game: &mut MainGame) {
        if game.can_swap == false || game.config.holding_enabled == false || game.game_over == true
        {
            return;
        }

        // If no piece is held, we just de-spawn the current piece.
        if game.held_piece.is_none() {
            for block in game.current_piece.orientations[game.current_piece.rotations].clone() {
                let temp_block_0 = block.0 + game.current_piece.offset.0;
                let temp_block_1 = block.1 + game.current_piece.offset.1;

                game.board.board[temp_block_0][temp_block_1] = ' ';
                game.board.color[temp_block_0][temp_block_1] = (255, 255, 255);
            }

            game.held_piece = Some(game.current_piece.clone());

            Self::spawn_piece(game.next_piece.clone(), game, true);

            let new_piece = Self::get_random_piece(&mut game.piece_bag, game.config.clone(), false);
            game.current_piece = game.next_piece.clone();
            Piece::spawn_piece(game.next_piece.clone(), game, true);
            match game.next_piece.piece_type {
                PieceType::I => game.piece_count[0] += 1,
                PieceType::L => game.piece_count[1] += 1,
                PieceType::J => game.piece_count[2] += 1,
                PieceType::S => game.piece_count[3] += 1,
                PieceType::Z => game.piece_count[4] += 1,
                PieceType::O => game.piece_count[5] += 1,
                PieceType::T => game.piece_count[6] += 1,
            }

            game.next_piece = new_piece;
        // If a piece is held, we de-spawn the current piece and spawn the old held piece.
        } else {
            let copied_piece = game.current_piece.clone();

            for block in game.current_piece.orientations[game.current_piece.rotations].clone() {
                let temp_block_0 = block.0 + game.current_piece.offset.0;
                let temp_block_1 = block.1 + game.current_piece.offset.1;

                game.board.board[temp_block_0][temp_block_1] = ' ';
                game.board.color[temp_block_0][temp_block_1] = (255, 255, 255);
            }

            let mut held_piece_copy = game.held_piece.clone().unwrap();
            held_piece_copy.offset = (0, 3);
            held_piece_copy.rotations = 0;

            game.current_piece = held_piece_copy.clone();
            Self::spawn_piece(held_piece_copy, game, true);
            game.held_piece = Some(copied_piece);
        }

        game.can_swap = false;
    }

    /// Rotates a piece, either clockwise or counter-clockwise.
    /// Returns a bool whether or not the rotation succeeded.
    pub fn rotate_piece(piece: &mut Piece, board: &mut Board, clockwise: bool) -> bool {
        let mut temp_rotation: usize;

        if clockwise == true {
            temp_rotation = piece.rotations + 1;

            if temp_rotation == 4 {
                temp_rotation = 0;
            }
        } else {
            if piece.rotations == 0 {
                temp_rotation = 3;
            } else {
                temp_rotation = piece.rotations - 1;
            }
        }

        // We first check if the rotation is valid, and return if not.
        for block in piece.orientations[temp_rotation].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            let mut temp_piece_pos: Vec<(usize, usize)> = vec![];

            for block in piece.orientations[piece.rotations].clone() {
                let temp_block_2 = block.0 + piece.offset.0;
                let temp_block_3 = block.1 + piece.offset.1;
                temp_piece_pos.push((temp_block_2, temp_block_3));
            }

            if temp_block_0 > 19
                || temp_block_1 > 9
                || (board.board[temp_block_0][temp_block_1] == '#'
                    && !temp_piece_pos.contains(&(temp_block_0, temp_block_1)))
            {
                return false;
            }
        }

        // And then we actually do it.
        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            board.board[temp_block_0][temp_block_1] = ' ';
            board.color[temp_block_0][temp_block_1] = (255, 255, 255);
        }

        if clockwise == true {
            piece.rotations += 1;

            if piece.rotations == 4 {
                piece.rotations = 0;
            }
        } else {
            if piece.rotations == 0 {
                piece.rotations = 3;
            } else {
                piece.rotations -= 1;
            }
        }

        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            board.board[temp_block_0][temp_block_1] = '#';
            board.color[temp_block_0][temp_block_1] = piece.color;
        }

        true
    }

    /// Moves a piece down a row.
    /// Returns a bool whether or not the move succeeded.
    pub fn move_piece_down(game: &mut MainGame, shadow: bool, ctx: &mut ggez::Context) -> bool {
        if game.game_over == true {
            return false;
        }

        for block in game.current_piece.orientations[game.current_piece.rotations].clone() {
            let temp_block_0 = block.0 + game.current_piece.offset.0;
            let temp_block_1 = block.1 + game.current_piece.offset.1;

            let mut temp_piece_pos: Vec<(usize, usize)> = vec![];

            for block in game.current_piece.orientations[game.current_piece.rotations].clone() {
                let temp_block_2 = block.0 + game.current_piece.offset.0;
                let temp_block_3 = block.1 + game.current_piece.offset.1;
                temp_piece_pos.push((temp_block_2, temp_block_3));
            }

            if temp_block_0 == 19
                || (game.board.board[temp_block_0 + 1][temp_block_1] == '#'
                    && !temp_piece_pos.contains(&(temp_block_0 + 1, temp_block_1)))
            {
                if shadow != true {
                    // If the piece cannot go any further and it is not a "shadow" piece
                    // we increase the score, level, check for full lines and so on.
                    let lines_erased = MainGame::erase_lines(
                        ctx,
                        &mut game.board,
                        &mut game.score,
                        game.level,
                        &mut game.clear_count,
                    ) as u128;

                    game.lines_cleared += lines_erased;

                    game.level = ((game.lines_cleared as f64 + 1.0) / 10.0).ceil() as u128;

                    let new_piece =
                        Self::get_random_piece(&mut game.piece_bag, game.config.clone(), false);
                    game.current_piece = game.next_piece.clone();
                    Piece::spawn_piece(game.next_piece.clone(), game, false);
                    game.next_piece = new_piece;

                    game.can_swap = true;
                    game.score += game.down_presses;
                }

                return false;
            }
        }

        for block in game.current_piece.orientations[game.current_piece.rotations].clone() {
            let temp_block_0 = block.0 + game.current_piece.offset.0;
            let temp_block_1 = block.1 + game.current_piece.offset.1;

            game.board.board[temp_block_0][temp_block_1] = ' ';
            game.board.color[temp_block_0][temp_block_1] = (255, 255, 255);
        }
        game.current_piece.offset.0 += 1;

        for block in game.current_piece.orientations[game.current_piece.rotations].clone() {
            let temp_block_0 = block.0 + game.current_piece.offset.0;
            let temp_block_1 = block.1 + game.current_piece.offset.1;

            game.board.board[temp_block_0][temp_block_1] = '#';
            game.board.color[temp_block_0][temp_block_1] = game.current_piece.color;
        }

        true
    }

    /// Drops a piece down as far as it will go.
    pub fn drop_piece_down(game: &mut MainGame, shadow: bool, ctx: &mut ggez::Context) {
        if game.game_over == true {
            return;
        }

        let mut drop_counter: u128 = 0;

        loop {
            let stop = Self::move_piece_down(game, shadow, ctx);
            drop_counter += 1;
            if stop == false {
                break;
            }
        }

        game.score += drop_counter;
    }

    /// Gets a "shadow" piece, which is the current piece, if it were dropped as far as it will go in the current position.
    /// This is just for drawing a shaded version of the piece on the board.
    pub fn get_shadow_piece(game: &mut MainGame, ctx: &mut ggez::Context) -> Vec<(usize, usize)> {
        let mut shadow_game = game.clone();

        Self::drop_piece_down(&mut shadow_game, true, ctx);

        let mut shadow_piece_coordinates: Vec<(usize, usize)> = Vec::new();

        for block in
            shadow_game.current_piece.orientations[shadow_game.current_piece.rotations].clone()
        {
            let temp_block_0 = block.0 + shadow_game.current_piece.offset.0;
            let temp_block_1 = block.1 + shadow_game.current_piece.offset.1;
            shadow_piece_coordinates.push((temp_block_0, temp_block_1));
        }

        shadow_piece_coordinates
    }

    /// Moves a piece left on the board.
    /// Returns a bool whether or not the move succeeded.
    pub fn move_piece_left(piece: &mut Piece, board: &mut Board) -> bool {
        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            let mut temp_piece_pos: Vec<(usize, usize)> = vec![];

            for block in piece.orientations[piece.rotations].clone() {
                let temp_block_2 = block.0 + piece.offset.0;
                let temp_block_3 = block.1 + piece.offset.1;
                temp_piece_pos.push((temp_block_2, temp_block_3));
            }

            if temp_block_1 == 0
                || (board.board[temp_block_0][temp_block_1 - 1] == '#'
                    && !temp_piece_pos.contains(&(temp_block_0, temp_block_1 - 1)))
            {
                return false;
            }
        }

        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            board.board[temp_block_0][temp_block_1] = ' ';
            board.color[temp_block_0][temp_block_1] = (255, 255, 255);
        }
        piece.offset.1 -= 1;

        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            board.board[temp_block_0][temp_block_1] = '#';
            board.color[temp_block_0][temp_block_1] = piece.color;
        }

        true
    }

    /// Moves a piece right on the board.
    /// Returns a bool whether or not the move succeeded.
    pub fn move_piece_right(piece: &mut Piece, board: &mut Board) -> bool {
        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            let mut temp_piece_pos: Vec<(usize, usize)> = vec![];

            for block in piece.orientations[piece.rotations].clone() {
                let temp_block_2 = block.0 + piece.offset.0;
                let temp_block_3 = block.1 + piece.offset.1;
                temp_piece_pos.push((temp_block_2, temp_block_3));
            }

            if temp_block_1 == 9
                || (board.board[temp_block_0][temp_block_1 + 1] == '#'
                    && !temp_piece_pos.contains(&(temp_block_0, temp_block_1 + 1)))
            {
                return false;
            }
        }

        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            board.board[temp_block_0][temp_block_1] = ' ';
            board.color[temp_block_0][temp_block_1] = (255, 255, 255);
        }
        piece.offset.1 += 1;

        for block in piece.orientations[piece.rotations].clone() {
            let temp_block_0 = block.0 + piece.offset.0;
            let temp_block_1 = block.1 + piece.offset.1;

            board.board[temp_block_0][temp_block_1] = '#';
            board.color[temp_block_0][temp_block_1] = piece.color;
        }

        true
    }
}
