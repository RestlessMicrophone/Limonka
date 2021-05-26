use crate::utils::world_generator::world_cell::MapCell;
use crate::utils::misc::dim_array::Array2d;

pub struct Worldmap {
    pub(crate) world_map_cells : Array2d<MapCell>
}