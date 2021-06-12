use crate::utils::world_generator;
use crate::utils::world_generator::world_cell::WalkCell;
use crate::utils::world_generator::indirect_world::MapCell;

///rewrite later to allow all sort of types
pub struct Array2d<T> {
    size: i64,
    pub arr_i: Vec<T>,
}

//rewrite later, cannot infer type for type parameter 'T'
impl<T> Array2d<T> {
    pub fn get_val_at(&mut self, x : &i64, y : &i64) -> &mut T{
        let sizeU = self.size.clone();
        let to_pick = (x * sizeU) + y;
        &mut self.arr_i[to_pick as usize]
    }

    pub fn get_val_at_index(&mut self, x : &i64) -> &mut T{
        let to_pick = x.clone() as usize;
        &mut self.arr_i[to_pick as usize]
    }

    pub fn size(&self) -> i64 {
        let toReturn : i64 = self.size.clone();
        toReturn
    }


    pub fn is_available(&mut self, x : &i64, y : &i64) -> bool{
        if (x > &(0 as i64)) & (x < &self.size ) & (y > &(0 as i64)) & (y < &self.size)
        {
            return true;
        }
        return false;
    }
}

/// in the future replace it with a generic type
/// common array operations

pub fn return2d_mapArray(size: i64) -> Array2d<MapCell> {

    let size_multidimensional = &size * &size;

    let mut arr_init = vec![MapCell{
        pos_x: 0,
        pos_y: 0,
        landType: world_generator::indirect_world::MapCell::initLand(),
        actionReady: false,
        massLeft: 0
    }];

    let mut x = 0;
    let mut y = 0;

    for n in 1..size_multidimensional {
        let to_add = world_generator::indirect_world::MapCell{
            pos_x: x.clone(),
            pos_y: y.clone(),
            landType: world_generator::indirect_world::MapCell::initLand(),
            actionReady: false,
            massLeft: 0
        };

        arr_init.push(to_add);
    }

    let arraytoinit = Array2d{
        size: size,
        arr_i: arr_init
    };

    arraytoinit
}

pub fn return2d_worldarray(size: i64) -> Array2d<WalkCell> {

    let size_multidimensional = &size * &size;

    let mut arr_init = vec![WalkCell {pos_x : 0, pos_y : 0}];

    let mut x = 0;
    let mut y = 0;

    for n in 1..size_multidimensional {
        let to_add = world_generator::world_cell::WalkCell{
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