use crate::utils::world_generator;
use crate::utils::world_generator::world_cell::MapCell;

/// rewrite later to allow all sort of types
pub struct Array2d<T> {
    size: i64,
    arr_i: Vec<T>,
}

//rewrite later, cannot infer type for type parameter 'T'
impl<T> Array2d<T> {
    pub fn get_val_at(&self, x : i64, y : i64) -> &T{
        let to_pick = (x * &self.size) + y;
        &self.arr_i[to_pick as usize]
    }
}

pub fn return2d_worldarray<T>(size: i64) -> Array2d<MapCell> {

    let size_multidimensional = &size * &size;

    let mut arr_init = vec![MapCell {pos_x : 0, pos_y : 0}];

    for n in 1..size_multidimensional {
        let to_add = world_generator::world_cell::MapCell{
            pos_x: 0,
            pos_y: 0
        };

        arr_init.push(to_add);
    }

    let arraytoinit = Array2d{
        size: size,
        arr_i: arr_init
    };

    arraytoinit
}