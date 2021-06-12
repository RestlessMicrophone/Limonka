//this is for the world map that is used for history or outside of players view.
use crate::utils::misc::dim_array::Array2d;
use raylib::core::math::Vector3;
use crate::utils::world_generator::indirect_world::typeOfLand::{greenlands, waterlands};
use raylib::color::Color;
use rand::Rng;

pub struct MapCell {
    pub(crate) pos_x: i64,
    pub(crate) pos_y: i64,
    pub(crate) landType : typeOfLand,
    pub(crate) actionReady : bool,
    pub massLeft : i32

}

impl MapCell {


    pub fn initLand() -> typeOfLand{
        waterlands
    }

    pub fn getPosX(&self) -> i64{

        let toReturnX = self.pos_x.clone();
        toReturnX
    }

    pub fn getPosY(&self) -> i64{

        let toReturnY = self.pos_y.clone();
        toReturnY
    }

    pub fn add_vector(&mut self, x: i64, y: i64){
        let addX = x.clone() as i64;
        let addY = y.clone() as i64;

        self.pos_x += addX;
        self.pos_y += addY;
    }
}

pub fn return_default_cell(pos_x: i64, pos_y: i64) -> MapCell {
    let to_return = MapCell {
        pos_x,
        pos_y,
        landType: typeOfLand::waterlands,
        actionReady: false,
        massLeft: 0
    };
    to_return
}


pub struct Worldmap {
    pub(crate) world_map_cells: Array2d<MapCell>

}


impl Worldmap {



    pub fn move_cells(&mut self){

        let cells_size = &self.world_map_cells.size() * &self.world_map_cells.size();

        for x in 0..cells_size{

            let current_cell = self.world_map_cells.get_val_at_index(&x);

            let mass_left : i32 = current_cell.massLeft.clone();

            if current_cell.actionReady {

            let looping_indexes : Vec<i64> = vec![
                &current_cell.pos_x -1, current_cell.pos_y.clone(),
                &current_cell.pos_x +1, current_cell.pos_y.clone(),
                current_cell.pos_x.clone(), &current_cell.pos_y - 1,
                current_cell.pos_x.clone(), &current_cell.pos_y + 1
            ];

                let looping_len = (looping_indexes.len() / 2) + 1;

            // loops through possible combinations
            for y in 0..looping_len{

                let indexX = looping_indexes.get(y).unwrap();
                let indexY = looping_indexes.get(y + 1).unwrap();


                if (self.world_map_cells.is_available(&indexX, &indexY)) {

                    let n_cell = self.world_map_cells.get_val_at(&indexX, &indexY);

                    n_cell.set_cell(mass_left.clone());

                }


            }



            }


            //current_cell.landType = typeOfLand::waterlands;

        }

    }

    pub fn organize_map(&mut self){

        let mut x = 0;
        let mut y = 0;


        let size = self.world_map_cells.size() as usize;

        let sizeLoop : i64 = (size.clone() * size.clone()) as i64;

        let mut x: i64 = 0;
        let mut y : i64 = 0;

        let mut toAddX : i64 = 0;
        let mut toAddY : i64 = 0;

        println!("the vector size is {}", &sizeLoop);


        let loopsize: i64 = (size.clone() - 1) as i64;

        for i in 0..sizeLoop {

            let mut current_block =  self.world_map_cells.get_val_at(&x, &y);

            current_block.add_vector(toAddX.clone(), toAddY.clone());

            if y < loopsize {
                y += 1;
                toAddY = y.clone() + 1;
            }

            else {
                //  current_block.add_vector(toAddX.clone(), 0);
                x += 1;
                y = 0;
                toAddX = x.clone() + 1;
                toAddY = 0;
            }

        }

    }

}


// cell stuff

pub enum typeOfLand {
    waterlands,
    greenlands
}

impl MapCell{

    // ugly, but don't know a better solution now. REWRITE THIS
    pub fn get_type_of_land(&self) -> typeOfLand{

        match self.landType {
            waterlands => {
                waterlands
            }
            greenlands => {
                greenlands
            }
        }


    }

    pub fn get_color(&self) -> Color{

        match self.landType {
            typeOfLand::waterlands => {
                Color::BLUE
            }
            greenlands => {
                Color::GREEN
            }
        }
    }


    pub fn set_cell(&mut self, mass_size : i32){
        self.landType = typeOfLand::greenlands;
        self.actionReady = true;
        self.massLeft = mass_size - 1;
    }

    pub fn active_cell(&mut self){

        //handled externally

        /*
        if self.morph_cell(self.get_type_of_land()) {

            let looping_indexes : Vec<i64> = vec![
                &self.pos_x -1, self.pos_y.clone(),
                &self.pos_x +1, self.pos_y.clone(),
                self.pos_x.clone(), &self.pos_y - 1,
                self.pos_x.clone(), &self.pos_y + 1
            ];

            let size = looping_indexes.len() / 2 as usize;

            for x in 0..size{

                let xval  : &i64 = looping_indexes.get(x).unwrap();
                let yval: &i64 = looping_indexes.get(x + 1).unwrap();

                if world_to_use.world_map_cells.is_available(xval, yval) {

                    let this_cell =  world_to_use.world_map_cells.get_val_at(xval, yval);

                    this_cell.massLeft = &self.massLeft - 1;

                    if this_cell.massLeft > 0 {
                        this_cell.actionReady = true;
                    }


                }

            }


            self.actionReady = false;


        }
*/


        

    }


    fn morph_cell(&mut self, typeOfLand : typeOfLand) -> bool{


        let mut rng = rand::thread_rng();
        let pass_chance = rng.gen_range(0..10);
        if pass_chance == 0{
            self.landType = greenlands;
            return true;
        }
        return false;

    }

}
