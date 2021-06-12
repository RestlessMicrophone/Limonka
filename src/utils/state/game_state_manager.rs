use std::borrow::Borrow;

mod menu_state;
pub(crate) mod game_state;

/// all game logic happens in states
pub fn apply_state(state_id: &i16){

    let (mut rl, mut thread) = raylib::init()
        .size(1366, 768)
        .title("Limonka")
        .build();

    // init every node important to the whole program below
    let asset_handler = crate::utils::data_handler::asset_handler::handler{
        handle: & mut (rl),
        thread: &mut thread
    };


    rl.set_target_fps(60);



    // init states
    let mut game_state = crate::utils::data_handler::state_initializer::init_game_state(&mut rl, &thread);

    &game_state.game_map.organize_map();

    &game_state.world_map.organize_map();

    &game_state.ECSworld.dummy_entity();

    &game_state.world_map.world_map_cells.get_val_at(&6,&6).set_cell(3);
    // heart of the pgrogram
    while !rl.window_should_close() {


        match state_id {
        // we do a bit of cheating for now
        0 => menu_state::update_state(),
        1 => game_state.update_state(&mut (rl), &(thread)),

        _ => {}
    }

    }
}

pub fn menu_state(){

}


