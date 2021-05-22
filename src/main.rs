mod utils;

use crate::utils::state::game_state_manager::state;
use crate::utils::state::game_state_manager::StateManager;
use raylib::prelude::*;


//// main loop method
fn main(){

    //// init && settings
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    //select_state();




    /// main loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }


}


fn select_state(){


}



