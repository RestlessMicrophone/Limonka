//this is for the world map that is used for history or outside of players view.
use crate::utils::misc::dim_array::Array2d;
use raylib::core::math::Vector3;
use crate::utils::world_generator::indirect_world::typeOfLand::{greenlands, waterlands};
use raylib::color::Color;
use rand::{Rng, thread_rng};

pub struct MapCell {
    pub(crate) pos_x: i64,
    pub(crate) pos_y: i64,
    pub(crate) landType: typeOfLand,
    pub(crate) actionReady: bool,
    pub(crate) action_set: bool,
    pub(crate) action_unavailable : bool,
    pub massLeft: i32,
}

impl MapCell {
    pub fn initLand() -> typeOfLand {
        waterlands
    }

    pub fn getPosX(&self) -> i64 {
        let toReturnX = self.pos_x.clone();
        toReturnX
    }

    pub fn getPosY(&self) -> i64 {
        let toReturnY = self.pos_y.clone();
        toReturnY
    }

    pub fn add_vector(&mut self, x: i64, y: i64) {
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
        action_set: false,
        action_unavailable: false,
        massLeft: 0,
    };
    to_return
}


pub struct Worldmap {
    pub(crate) world_map_cells: Array2d<MapCell>,
    pub(crate) map_update_time: f32,
}


impl Worldmap {

    pub fn let_there_be_light(&mut self){

        let mut rng = rand::thread_rng();

        let island_num = rng.gen_range(5.. 35);

        for i in 0..island_num {

            let x_start = rng.gen_range(0..self.world_map_cells.size());

            let y_start = rng.gen_range(0..self.world_map_cells.size());

            let start_cell = self.world_map_cells.get_val_at(&x_start, &y_start);

            let cell_span = rng.gen_range(50..250);

            start_cell.set_cell(cell_span, typeOfLand::get_random());
        }

    }



    pub fn move_cells(&mut self) {
        if self.map_update_time < 0.0 {
            let cells_size = &self.world_map_cells.size() * &self.world_map_cells.size();

            // cleaning up the cells to be ready for actions
            for x in 0..cells_size {
                let current_cell = self.world_map_cells.get_val_at_index(&x);

                if current_cell.action_set {
                    current_cell.actionReady = true;
                    current_cell.action_set = false;
                }
            }


            let cells_size = &self.world_map_cells.size() * &self.world_map_cells.size();

            for x in 0..cells_size {

                let current_cell = self.world_map_cells.get_val_at_index(&x);

                let current_cell_type = current_cell.get_type_of_land();

                let mut mass_left: i32 = current_cell.massLeft.clone();

                mass_left -= 1;

                if (current_cell.actionReady) & (current_cell.massLeft > 0){

                    current_cell.actionReady = false;

                    let looping_indexes: Vec<i64> = vec![
                        &current_cell.pos_x - 1, &current_cell.pos_x + 1, current_cell.pos_x.clone(), current_cell.pos_x.clone()
                    ];

                    let looping_indexes2: Vec<i64> = vec![
                        current_cell.pos_y.clone(), current_cell.pos_y.clone(), &current_cell.pos_y - 1, &current_cell.pos_y + 1
                    ];

                    //println!("CC, Action: {}, Mass {}, posX{}, posY{}", &current_cell.actionReady, mass_left, &current_cell.pos_x, &current_cell.pos_y);

                    //println!("=============");

                    let mut prev_roll = 0;

                    // loops through possible combinations
                    for y in 0..looping_indexes.len() {

                        let indexX = looping_indexes.get(y).unwrap();
                        let indexY = looping_indexes2.get(y).unwrap();

                        //println!("the vector is: {}|{},   the mass is {}", &indexX, &indexY, &mass_left);

                        let pass_chance = MapCell::morph_cell(&current_cell_type, &prev_roll);

                        if ((self.world_map_cells.is_available(&indexX, &indexY)) & (pass_chance)) {

                            prev_roll += 1;

                            let n_cell = self.world_map_cells.get_val_at(&indexX, &indexY);

                            n_cell.set_cell(mass_left.clone(), current_cell_type.clone());

                        }
                    }
                }

                // release the mass
            }
        }
    }

    pub fn organize_map(&mut self) {
        let mut x = 0;
        let mut y = 0;


        let size = self.world_map_cells.size() as usize;

        let sizeLoop: i64 = (size.clone() * size.clone()) as i64;

        let mut x: i64 = 0;
        let mut y: i64 = 0;

        let mut toAddX: i64 = 0;
        let mut toAddY: i64 = 0;

        let loopsize: i64 = (size.clone()) as i64;

        for i in 0..sizeLoop {
            let mut current_block = self.world_map_cells.get_val_at(&x, &y);

            current_block.add_vector(toAddX.clone(), toAddY.clone());

            if y < loopsize {
                y += 1;
                toAddY = y.clone();
            } else {
                //  current_block.add_vector(toAddX.clone(), 0);
                x += 1;
                y = 0;
                toAddX = x.clone();
                toAddY = 0;
            }
        }
    }

    pub fn update_time(&mut self, delta_time: &f32) {
        let nullP: f32 = 0.0;

        if &self.map_update_time > &nullP {
            self.map_update_time -= delta_time;
        } else {
            self.map_update_time = 0.01;
        }
    }
}


// cell stuff


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum typeOfLand {
    waterlands,
    greenlands,
    snowlands,
}

impl typeOfLand {

    pub fn get_random() -> typeOfLand{

        let mut rng = rand::thread_rng();

        let mut pick = rng.gen_range(1..3);

        println!("the num {} has been picked", &pick);

        match pick {
            1 => {
                typeOfLand::greenlands
            }
            2 => {
                typeOfLand::snowlands
            }
            _ => {
                typeOfLand::greenlands
            }
        }


    }



}


impl MapCell {
    // ugly, but don't know a better solution now. REWRITE THIS
    pub fn get_type_of_land(&self) -> typeOfLand {
        match self.landType {
            waterlands => {
                waterlands
            }
            greenlands => {
                greenlands
            }
            typeOfLand::snowlands => {
                typeOfLand::snowlands
            }
        }
    }

    pub fn get_color(&self) -> Color {
        match self.landType {
            typeOfLand::waterlands => {
                Color::BLUE
            }
            greenlands => {
                Color::GREEN
            }
            typeOfLand::snowlands => {
                Color::LIGHTGRAY
            }
        }
    }


    pub fn set_cell(&mut self, mass_size: i32, land_type: typeOfLand) {
            self.landType = land_type;
            self.action_set = true;
            self.actionReady = false;
            self.massLeft = mass_size;
    }

    pub fn active_cell(&mut self) {

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


    fn morph_cell(typeOfLand: &typeOfLand, succesfull_rolls: &i32) -> bool {

        if typeOfLand != &typeOfLand::waterlands {

            let mut rng = rand::thread_rng();

            let mut mutation_chance = rng.gen_range(1..10);

            let cluster_var = rng.gen_range(0..succesfull_rolls.clone() + 1);

            let cluster_roll = (succesfull_rolls - cluster_var);

            mutation_chance += cluster_roll;

            let pass_chance = rng.gen_range(0..mutation_chance);

            if pass_chance == 0 {
                return true;
            }
        }

        return false;
    }


    fn get_land_type_morph_chance(typeOfLand: &typeOfLand){

    }




}
