use crate::utils::world_generator;
use crate::utils::world_generator::world_cell::MapCell;

///rewrite later to allow all sort of types
pub struct Array2d<T> {
    size: i64,
    arr_i: Vec<T>,
}

//rewrite later, cannot infer type for type parameter 'T'
impl<T> Array2d<T> {
    pub fn get_val_at(&mut self, x : &usize, y : &usize) -> &mut T{
        let sizeU = self.size.clone() as usize;
        let to_pick = (x * sizeU) + y;
        &mut self.arr_i[to_pick as usize]
    }

    pub fn size(&self) -> i64 {
        let toReturn : i64 = self.size.clone();
        toReturn
    }
}

pub fn return2d_worldarray(size: i64) -> Array2d<MapCell> {

    let size_multidimensional = &size * &size;

    let mut arr_init = vec![MapCell {pos_x : 0, pos_y : 0}];

    let mut x = 0;
    let mut y = 0;

    for n in 1..size_multidimensional {
        let to_add = world_generator::world_cell::MapCell{
            pos_x: x.clone(),
            pos_y: y.clone()
        };

        arr_init.push(to_add);
    }

    let arraytoinit = Array2d{
        size: size,
        arr_i: arr_init
    };

    arraytoinit
}