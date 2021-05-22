mod utils;

use crate::utils::state::game_state_manager;

fn main(){

    //// start up screen
    let mut state_num : i16 = 1;

    game_state_manager::apply_state(&state_num);

}



