mod game;
use game::Game;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(Game::WIDTH as i32,Game::HEIGHT as i32)
        .title("Snake game")
        .vsync()
        .build();

    let mut game = Game::new(&mut rl, &thread);

    rl.set_target_fps(8);
    

    while !rl.window_should_close() {
        /*------UPDATE---------- */

        game.update(&mut rl);

        /*------DRAW------ */
        let mut d = rl.begin_drawing(&thread);

        game.draw(&mut d);
    }
}