use crate::utils::input::input_map;
use raylib::core::drawing::{RaylibDraw, RaylibMode3DExt, RaylibDraw3D};
use raylib::color::Color;
use raylib::core::math::Vector3;
use raylib::{RaylibHandle, RaylibThread};
use raylib::camera::Camera3D;
use crate::utils::world_generator::world::Worldmap;


pub struct game_state{

    pub(crate) camera: Camera3D,
    pub(crate) cube_position: Vector3,
    pub(crate) is_state_running: bool,
    pub(crate) world_map: Worldmap

}

impl game_state {


    pub fn update_state(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread){

        // input
        unsafe {
            input_map::handle_keys(&rl);
            input_map::handle_mouse(&rl);
        }

        //logic

        // render
        rl.update_camera(&mut self.camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut d = d.begin_mode3D(&self.camera);


            for x in 0..self.world_map.world_map_cells.size() {
                for y in 0..self.world_map.world_map_cells.size() {

                    let xToPlace = x.clone() as usize;
                    let yToPlace = y.clone() as usize;

                    let drawpos = &self.world_map.world_map_cells.get_val_at(&xToPlace, &yToPlace).get3DVector();

                    d.draw_cube(drawpos, 2.0, 2.0, 2.0, Color::RED);
                }
            }
        }

        d.draw_text("Limonka 0.1", 12, 12, 20, Color::BLACK);
    }

}





