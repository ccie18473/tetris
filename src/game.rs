use crate::prelude::*;

pub const GAME_WIDTH: usize = 10;
pub const GAME_HEIGHT: usize = 20;

pub const BLOCK_SIZE: isize = 25;
pub const GAME_SPEED: usize = 10;

#[derive(Debug, PartialEq)]
pub enum GameState {
    GsGameOver,      // game over, wait for newgame command
    GsBlockDropping, // normal mode, blocks are dropping
    GsPaused,        // game is paused
}
pub struct TGame {
    pub blocks: Vec<TBlock>,
    pub game_state: GameState, // current state of the game
    pub current_block: TBlock, // currently falling block
    pub x: isize,              // current block position
    pub y: isize,
    pub color: usize, // current block color
    //let mut x = [[0.0; N] ; M]; M Rows, N Columns
    pub board: [[isize; GAME_WIDTH + 2]; GAME_HEIGHT + 1], // the game grid
    pub timer: usize,                                      // ID of game timer
    pub drop_count: usize, // countdown to next time piece falls one row
    pub dropping: bool,    // is the user holding the 'drop' key down?
    pub game_speed: usize, // read from .INI file
    pub block_size: isize,
    pub score: usize,
    pub duration: f32,
}

impl TGame {
    pub fn new(_ctx: &mut Context) -> Self {
        let blocks = TBlocks::new();
        Self {
            blocks,
            game_state: GameState::GsGameOver,
            current_block: TBlock::new(),
            x: 0,
            y: 0,
            color: 0,
            board: [[0; GAME_WIDTH + 2]; GAME_HEIGHT + 1],
            timer: 0,
            drop_count: 0,
            dropping: false,
            game_speed: GAME_SPEED,
            block_size: BLOCK_SIZE,
            score: 0,
            duration: 0.0,
        }
    }
    // NewGame -- start a new game.  First clear the board, then
    // add the first block, and change the game state to gsBlockDropping
    //
    pub fn new_game(&mut self) {
        //println!("new_game");
        qrand::srand(miniquad::date::now() as _);

        self.clear_board();
        self.new_block(qrand::gen_range::<usize>(0, 7));
        self.game_state = GameState::GsBlockDropping;
        //Invalidate();
    }
    // ClearBoard -- resets the game board to be empty
    //
    pub fn clear_board(&mut self) {
        println!("clear_board");
        for i in 0..GAME_HEIGHT + 1 {
            for j in 0..GAME_WIDTH + 2 {
                self.board[i][j] = 0;
            }
        }
        for i in 0..GAME_HEIGHT + 1 {
            self.board[i][0] = -1;
            self.board[i][GAME_WIDTH + 1] = -1;
        }
        for j in 0..GAME_WIDTH + 2 {
            self.board[GAME_HEIGHT][j] = -1;
        }
    }
    // NewBlock -- creates a new block at the top of the screen
    //
    pub fn new_block(&mut self, block_type: usize) {
        //println!("new_block");
        self.current_block = self.blocks[block_type];
        self.color = block_type + 1;
        self.x = 4;
        self.y = 0;
        // if the new block hits anything on the screen, the game is over
        //
        if self.hit_test(self.current_block, self.x, self.y) {
            self.place_block();
            //Invalidate();
            self.game_state = GameState::GsGameOver;
        }
    }
    // HitTest -- tests to see if a block overlaps any occupied square
    //            on the board
    //
    pub fn hit_test(&mut self, block: TBlock, x: isize, y: isize) -> bool {
        //println!("hit_test");
        for i in 0..4 {
            for j in 0..4 {
                if (x + i) < (GAME_WIDTH + 1) as isize
                    && (x + i + 1) >= 0
                    && (y + j) < (GAME_HEIGHT + 1) as isize
                    && (y + j) >= 0
                // make sure block in question is in range
                {
                    if self.board[(y + j) as usize][(x + 1 + i) as usize] != 0 {
                        // if the board piece is empty, skip test
                        if block.elements[(j * 4 + i) as usize] == '*' {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
    // PlaceBlock -- puts the current block permanently into the board array.
    //               this function is called when the block reaches the bottom
    //               of the game board.
    //
    pub fn place_block(&mut self) {
        //println!("place_block");
        for i in 0..4_isize {
            for j in 0..4_isize {
                if self.current_block.elements[(j * 4 + i) as usize] == '*' {
                    self.board[(self.y + j) as usize][(self.x + 1 + i) as usize] =
                        self.color as isize;
                }
            }
        }
        //Invalidate();
        self.remove_lines();
    }
    // RemoveLines -- checks for completed lines and removes them from
    // the game board.  If lines are removed, higher lines are moved down
    // to fill in the space.
    //
    pub fn remove_lines(&mut self) {
        //println!("remove_lines");
        let mut line_full: bool;
        let mut lines_removed: bool;

        lines_removed = false;

        let mut j = GAME_HEIGHT - 1;

        while j > 0 {
            line_full = true;
            for i in 1..=GAME_WIDTH {
                if self.board[j][i] == 0 {
                    line_full = false;
                }
            }
            if line_full {
                lines_removed = true;
                self.score += 1;
                for k in (1..=j).rev() {
                    for l in 1..=GAME_WIDTH {
                        self.board[k][l] = self.board[k - 1][l]
                    }
                }
                for l in 1..=GAME_WIDTH {
                    self.board[0][l] = 0;
                }
            } else {
                j -= 1;
            }
        }
        if lines_removed {
            if self.score == 10 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score == 20 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score == 30 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score == 40 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score == 50 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score == 60 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score == 70 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score == 80 && self.game_speed > 1 {
                self.game_speed -= 1;
            } else if self.score >= 90 && self.game_speed > 1 {
                self.game_speed -= 1;
            }
        }
    }
    // Pause -- handler for the 'Pause' menu item
    //
    pub fn pause(&mut self) {
        //println!("pause");
        if self.game_state == GameState::GsBlockDropping {
            self.game_state = GameState::GsPaused;
        } else {
            if self.game_state == GameState::GsPaused {
                self.game_state = GameState::GsBlockDropping;
            }
        }
        //Invalidate();
    }
    // DrawBlock -- draws an individual game piece.
    //
    pub fn draw_block(
        &mut self,
        ctx: &mut Context,
        block: TBlock,
        x: isize,
        y: isize,
    ) -> GameResult {
        //println!("draw_block");
        let size = block.size;
        for i in 0..size {
            for j in 0..size {
                if block.elements[(j * 4 + i) as usize] == '*' {
                    let block = graphics::Rect::new(
                        (x * self.block_size + i * self.block_size) as f32,
                        (y * self.block_size + j * self.block_size) as f32,
                        (self.block_size) as f32,
                        (self.block_size) as f32,
                    );
                    let r1 = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(1.0),
                        block,
                        PEN[self.color],
                    )?;
                    graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
                    let r2 = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        block,
                        BRUSH[self.color],
                    )?;
                    graphics::draw(ctx, &r2, graphics::DrawParam::default())?;
                }
            }
        }
        Ok(())
    }
    // Paint -- redraws the entire window.  Currently, whenever anything
    // on the game board changes, we redraw the entire board.  This could
    // obviously be improved, by only redrawing the area around the moving
    // block.
    //
    pub fn paint(&mut self, ctx: &mut Context) -> GameResult {
        //println!("paint");

        // if the game is paused, blank the screen (to prevent cheating!)

        if self.game_state == GameState::GsPaused {
            let pause_msg = graphics::Text::new(" * * P A U S E D * * ");
            let p = cgmath::Point2::new(
                2.0 * BLOCK_SIZE as f32,
                (GAME_HEIGHT as isize * BLOCK_SIZE / 2) as f32,
            );
            graphics::draw(ctx, &pause_msg, (p,))?;
        }

        // draw the permanent blocks

        for j in 0..GAME_HEIGHT {
            for i in 0..GAME_WIDTH {
                if self.board[j][i + 1] != 0 {
                    let permanent = graphics::Rect::new(
                        (i * self.block_size as usize) as f32,
                        (j * self.block_size as usize) as f32,
                        (self.block_size) as f32,
                        (self.block_size) as f32,
                    );
                    let r1 = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(1.0),
                        permanent,
                        PEN[(self.board[j][i + 1]) as usize],
                    )?;
                    graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
                    let r2 = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        permanent,
                        BRUSH[(self.board[j][i + 1]) as usize],
                    )?;
                    graphics::draw(ctx, &r2, graphics::DrawParam::default())?;
                }
            }
        }

        // display the game over message if the game has ended

        if self.game_state == GameState::GsGameOver {
            let pause_msg = graphics::Text::new("* G A M E   O V E R *");
            let p = cgmath::Point2::new(
                2.0 * BLOCK_SIZE as f32,
                (GAME_HEIGHT as isize * BLOCK_SIZE / 2) as f32,
            );
            graphics::draw(ctx, &pause_msg, (p,))?;
        }

        // if a block is dropping, draw it

        if self.game_state == GameState::GsBlockDropping {
            self.draw_block(ctx, self.current_block, self.x, self.y)
                .unwrap();
        }
        Ok(())
    }
    pub fn status_bar_render(&mut self, ctx: &mut Context) -> GameResult {
        let status_bar = graphics::Rect::new(
            0.0,
            (GAME_HEIGHT as isize * BLOCK_SIZE) as f32,
            (GAME_WIDTH as isize * BLOCK_SIZE) as f32,
            (4 * BLOCK_SIZE) as f32,
        );
        let r = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            status_bar,
            Color::BLACK)?;
        graphics::draw(ctx, &r, graphics::DrawParam::default())?;

        Ok(())
    }
    pub fn fps_render(&mut self, ctx: &mut Context) -> GameResult {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));
        let p = cgmath::Point2::new(
            3.0 * BLOCK_SIZE as f32,
            (GAME_HEIGHT as isize * BLOCK_SIZE + 25) as f32,
        );
        graphics::draw(ctx, &fps_display, (p,))?;

        Ok(())
    }
    pub fn duration_render(&mut self, ctx: &mut Context) -> GameResult {
        let secs_display = graphics::Text::new(format!("Duration: {:.1}", self.duration));
        let p = cgmath::Point2::new(
            3.0 * BLOCK_SIZE as f32,
            (GAME_HEIGHT as isize * BLOCK_SIZE + 50) as f32,
        );
        graphics::draw(ctx, &secs_display, (p,))?;

        Ok(())
    }
    pub fn score_render(&mut self, ctx: &mut Context) -> GameResult {
        let score_display = graphics::Text::new(format!("Score: {}", self.score));
        let p = cgmath::Point2::new(
            3.0 * BLOCK_SIZE as f32,
            (GAME_HEIGHT as isize * BLOCK_SIZE + 75) as f32,
        );
        graphics::draw(ctx, &score_display, (p,))?;

        Ok(())
    }
}
