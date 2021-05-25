mod utils;

use crate::utils::state::game_state_manager;
use crate::utils::world_generator::world;

fn main(){

    //// start up screen
    let mut state_num : i16 = 1;

    //world::main();
    game_state_manager::apply_state(&state_num);

}



