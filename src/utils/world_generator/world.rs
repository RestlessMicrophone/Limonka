use crate::utils::world_generator::world_cell::WalkCell;
use crate::utils::misc::dim_array::Array2d;

pub struct Gamemap {
    pub(crate) world_walk_cells: Array2d<WalkCell>
}


impl Gamemap {

    pub fn organize_map(&mut self){

        let mut x = 0;
        let mut y = 0;

        //probably rewrite this later since loops inside loops are for weirdos who don't shower
        //done?
        //now it's still ugly, but it will do for now

        let size = self.world_walk_cells.size();

        let sizeLoop = size.clone() * size.clone();

        let mut x: i64 = 0;
        let mut y : i64 = 0;

        let mut toAddX : i64 = 0;
        let mut toAddY : i64 = 0;

        println!("the vector size is {}", &sizeLoop);


        let loopsize: i64 = size.clone() - 1;

        for i in 0..sizeLoop {

            let mut current_block =  self.world_walk_cells.get_val_at(&x, &y);

            current_block.add_vector(toAddX.clone(), toAddY.clone());

            if y < loopsize {
                y += 1;
                toAddY = 3 * &y;
            }

            else {
              //  current_block.add_vector(toAddX.clone(), 0);
                x += 1;
                y = 0;
                toAddX = 3 * &x;
                toAddY = 0;
            }



        }

    }

}