mod game;
use game::Game;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(Game::WIDTH as i32, Game::HEIGHT as i32)
        .title("Snake game")
        .vsync()
        .build();

    let raylib_audio = Box::new(RaylibAudio::init_audio_device().unwrap());
    let raylib_audio: &'static RaylibAudio = Box::leak(raylib_audio);
    let mut game = Game::new(&mut rl, &thread, &raylib_audio);

    rl.set_target_fps(8);

    while !rl.window_should_close() {
        /*------UPDATE---------- */

        game.update(&mut rl);

        /*------DRAW------ */
        let mut d = rl.begin_drawing(&thread);

        game.draw(&mut d);
    }
}
