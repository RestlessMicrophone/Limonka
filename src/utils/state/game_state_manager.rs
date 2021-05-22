use std::borrow::Borrow;

mod menu_state;
mod game_state;

/// all game logic happens in states
pub fn apply_state(state_id: &i16){

    match state_id {
        0 => menu_state::update_state(),
        1 => game_state::update_state(),

        _ => {}
    }


}

pub fn menu_state(){

}


