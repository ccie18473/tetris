extern crate good_web_game as ggez;

mod blocks;
mod colors;
mod game;

mod prelude {
    pub use crate::blocks::*;
    pub use crate::colors::*;
    pub use crate::game::*;
    pub use cgmath::prelude::*;
    pub use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
    pub use ggez::graphics::{self, Color, Rect};
    pub use ggez::timer;
    pub use ggez::{Context, GameResult};
    pub use quad_rand as qrand;
    pub use std::env;
    pub use std::path;
    pub use std::time::{Duration, SystemTime};
    pub use std::{thread, time};
}

use prelude::*;

struct MainState {
    game: TGame,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (_w, _h) = graphics::drawable_size(ctx);
        let mut game = TGame::new(ctx);
        game.new_game();

        let s = MainState { game };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // This runs MAX_FPS time per second

        const INTERVAL: u32 = 10;

        while timer::check_update_time(ctx, INTERVAL) {
            match self.game.game_state {
                GameState::GsGameOver =>
                // no game in progress, do nothing
                {
                    ()
                }

                GameState::GsBlockDropping => {
                    self.game.duration += 0.1;
                    // game in progress
                    self.game.drop_count += 1; // increment drop counter
                    if self.game.dropping || self.game.drop_count == self.game.game_speed {
                        // if time to drop
                        self.game.drop_count = 0; // reset counter
                        self.game.y += 1; // move block down
                        qrand::srand(miniquad::date::now() as _);

                        if self
                            .game
                            .hit_test(self.game.current_block, self.game.x, self.game.y)
                        {
                            // if it hit something
                            self.game.y -= 1; // move it back up
                            self.game.place_block(); // make it permanent
                            self.game.remove_lines();
                            self.game.new_block(qrand::gen_range::<usize>(0, 7));
                        }
                        //Invalidate();                          // redraw game board
                    }
                }

                GameState::GsPaused =>
                // game is paused
                {
                    ()
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // This runs MAX_FPS time per second

        graphics::clear(
            ctx,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.25,
                a: 1.0,
            },
        );

        self.game.paint(ctx).unwrap();
        self.game.status_bar_render(ctx).unwrap();
        self.game.fps_render(ctx).unwrap();
        self.game.duration_render(ctx).unwrap();
        self.game.score_render(ctx).unwrap();

        graphics::present(ctx)?;

        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, _w: f32, _h: f32) {}

    // EvKeyUp/EvKeyDown -- respond to key press/release messages.  For the
    // 'drop' key, we set a flag whenever the key is held down.  For the
    // left/right/rotate keys, we only keep track of keydown events.
    //

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Down => {
                self.game.dropping = false;
            }
            _ => (), // Do nothing
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if self.game.game_state == GameState::GsPaused {
            self.game.game_state = GameState::GsBlockDropping;
            //Invalidate();
            return;
        }

        match keycode {
            // move block left
            KeyCode::Left => {
                self.game.x -= 1;

                if self
                    .game
                    .hit_test(self.game.current_block, self.game.x, self.game.y)
                {
                    self.game.x += 1;
                } else {
                    //Invalidate();
                    return;
                }
            }

            // move block right
            KeyCode::Right => {
                self.game.x += 1;

                if self
                    .game
                    .hit_test(self.game.current_block, self.game.x, self.game.y)
                {
                    self.game.x -= 1;
                } else {
                    //Invalidate();
                    return;
                }
            }
            // turn on fast dropping
            KeyCode::Down => {
                self.game.dropping = true;
                return;
            }
            // rotate block
            KeyCode::Space => {
                self.game.current_block.rotate();

                if self
                    .game
                    .hit_test(self.game.current_block, self.game.x, self.game.y)
                {
                    self.game.current_block.rotate();
                    self.game.current_block.rotate();
                    self.game.current_block.rotate();
                }
                //Invalidate();
                return;
            }

            _ => (), // Do nothing
        }
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        // pause game or start a new one
        self.game.pause();
        if self.game.game_state == GameState::GsGameOver {
            self.game.new_game();
        }
    }
}
pub fn main() -> GameResult {
    //let state = MainState::new(&mut context).unwrap();

    let conf = ggez::conf::Conf::default().window_title("Tetris v1.0.0, 2022".to_string());

    ggez::start(conf, |mut context| {
        Box::new(MainState::new(&mut context).unwrap())
    })
}
