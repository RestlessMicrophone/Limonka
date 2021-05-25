use raylib::{RaylibHandle, RaylibThread};
use raylib::core::math::Vector3;
use crate::utils::state::game_state_manager::game_state::game_state;
use crate::utils::world_generator::world::worldmap;
use ndarray::{Array2, Array, arr2};
use crate::utils::world_generator::world_cell::map_cell;

pub fn init_game_state(rl: &mut RaylibHandle, thread: &RaylibThread) -> game_state {
    let game_state_to_return = crate::utils::state::game_state_manager::game_state::game_state{
        camera: crate::utils::camera::topdown_camera::init_camera(rl),
        cube_position:  Vector3::zero(),
        is_state_running: true,
        world_map: worldmap_load()
    };
    game_state_to_return
}

fn worldmap_load() -> worldmap{

    /// make this array values filled with mapcells instances
    let array_to_use : Array2<map_cell> =  arr2(&[[1.,2.,3.], [4.,5.,6.]]);

    let world_to_generate = worldmap {
        world_map_cells: array_to_use
    };

    world_to_generate
}