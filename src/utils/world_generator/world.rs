use ndarray::Array2;
use crate::utils::world_generator::world_cell::map_cell;

pub struct worldmap{

    pub(crate) world_map_cells : Array2<map_cell>
}