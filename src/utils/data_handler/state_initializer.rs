use raylib::{RaylibHandle, RaylibThread};
use raylib::core::math::Vector3;
use crate::utils::state::game_state_manager::game_state::game_state;
use crate::utils::world_generator::world::Gamemap;
use ndarray::{Array2, Array, arr2};
use crate::utils::world_generator::world_cell::WalkCell;
use crate::utils::misc::dim_array;
use crate::utils::data_handler::ECShandler;
use crate::utils::world_generator::indirect_world::{MapCell, Worldmap};

pub fn init_game_state(rl: &mut RaylibHandle, thread: &RaylibThread) -> game_state {
    let game_state_to_return = crate::utils::state::game_state_manager::game_state::game_state{
        camera: crate::utils::camera::topdown_camera::init_camera(rl),
        cube_position:  Vector3::zero(),
        is_state_running: true,
        world_map: worldmap_load(155),
        game_map: gamemap_load(30),
        ECSworld: ECShandler::return_handler(),

    };
    game_state_to_return
}

/*
Things that should be initialized!!!

Every topdown functions that the state will need will be placed here.

 */

// game state ||||||

fn gamemap_load(size: i64) -> Gamemap {

    // make this array values filled with walk cell instances

    let world_to_generate = Gamemap {
        world_walk_cells: dim_array::return2d_worldarray(size)
    };

    world_to_generate
}

fn worldmap_load(size: i64) -> Worldmap {

    // make this array values filled with walk cell instances

    let world_to_generate = Worldmap {
        world_map_cells: dim_array::return2d_mapArray(size),
        map_update_time: 0.0
    };

    world_to_generate
}