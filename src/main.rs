mod game;
use game::Game;
use once_cell::sync::Lazy;
use raylib::prelude::*;

static RAYLIB_AUDIO: Lazy<RaylibAudio> = Lazy::new(|| RaylibAudio::init_audio_device().unwrap());
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(Game::WIDTH as i32, Game::HEIGHT as i32)
        .title("Snake game")
        .vsync()
        .build();

    let mut game = Game::new(&mut rl, &thread, &RAYLIB_AUDIO);

    rl.set_target_fps(8);

    while !rl.window_should_close() {
        /*------UPDATE---------- */

        game.update(&mut rl);

        /*------DRAW------ */
        let mut d = rl.begin_drawing(&thread);

        game.draw(&mut d);
    }
}
