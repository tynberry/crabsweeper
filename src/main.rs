pub mod game;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("Crabsweeper")]
async fn main() {
    //načti font
    let font = load_ttf_font("font.ttf").await.unwrap();
    //vytvoř hru
    let mut game = Game::new(10, 10, 50);
    game.fix_numbers();

    loop {
        clear_background(BLACK);
    
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        game.render(center_x, center_y, &font);

        next_frame().await;
    }
}
