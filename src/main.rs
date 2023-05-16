pub mod game;

use macroquad::prelude::*;

#[macroquad::main("Crabsweeper")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
