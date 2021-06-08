use raylib::{RaylibHandle, RaylibThread};
use raylib::core::math::Vector3;
use crate::utils::state::game_state_manager::game_state::game_state;
use crate::utils::world_generator::world::Worldmap;
use ndarray::{Array2, Array, arr2};
use crate::utils::world_generator::world_cell::MapCell;
use crate::utils::misc::dim_array;

pub fn init_game_state(rl: &mut RaylibHandle, thread: &RaylibThread) -> game_state {
    let game_state_to_return = crate::utils::state::game_state_manager::game_state::game_state{
        camera: crate::utils::camera::topdown_camera::init_camera(rl),
        cube_position:  Vector3::zero(),
        is_state_running: true,
        world_map: worldmap_load(30)
    };
    game_state_to_return
}

/*
Things that should be initialized!!!

Every topdown functions that the state will need will be placed here.

 */

fn worldmap_load(size: i64) -> Worldmap {

    // make this array values filled with mapcells instances

    let world_to_generate = Worldmap {
        world_map_cells: dim_array::return2d_worldarray(size)
    };

    world_to_generate
}
