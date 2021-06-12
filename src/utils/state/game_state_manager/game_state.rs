use crate::utils::input::input_map;
use raylib::core::drawing::{RaylibDraw, RaylibMode3DExt, RaylibDraw3D};
use raylib::color::Color;
use raylib::core::math::Vector3;
use raylib::{RaylibHandle, RaylibThread};
use raylib::camera::Camera3D;
use crate::utils::world_generator::world::Gamemap;
use crate::utils::data_handler::ECShandler;
use crate::utils::data_handler::ECShandler::eHandler;
use crate::utils::world_generator::indirect_world::{MapCell, Worldmap};
use core::time;
use std::thread;


pub struct game_state{

    pub(crate) camera: Camera3D,
    pub(crate) cube_position: Vector3,
    pub(crate) is_state_running: bool,
    pub(crate) world_map: Worldmap,
    pub(crate) game_map: Gamemap,
    pub ECSworld : eHandler

}

impl game_state {

    pub fn update_state(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread){
        // input
        unsafe {
            input_map::handle_keys(&rl);
            input_map::handle_mouse(&rl);
        }

        //logic

        self.game_logic();

        // render
        rl.update_camera(&mut self.camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut d = d.begin_mode3D(&self.camera);



            // rewrite later the render methods to be on just one loop

            //3D RENDER

            for x in 0..self.game_map.world_walk_cells.size() {
                for y in 0..self.game_map.world_walk_cells.size() {

                    let drawpos = &self.game_map.world_walk_cells.get_val_at(&x, &y).get3DVector();

                    d.draw_cube(drawpos, 2.0, 2.0, 2.0, Color::RED);
                }
            }
        }

        // 2D Render

        for x in 0..self.world_map.world_map_cells.size() {
            for y in 0..self.world_map.world_map_cells.size() {

                let current_cell = &mut self.world_map.world_map_cells.get_val_at(&x, &y);

                d.draw_rectangle((current_cell.getPosX() + 100) as i32, (current_cell.getPosY() + 100) as i32, 1, 1, current_cell.get_color());

            }
        }


        d.draw_text("Limonka 0.1", 12, 12, 20, Color::BLACK);
    }


    fn game_logic(&mut self){

        self.world_map.move_cells();


    }


}







