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

            d.draw_cube(self.cube_position, 2.0, 2.0, 2.0, Color::RED);
        }

        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }

}





