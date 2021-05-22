use crate::utils::input::input_map;
use raylib::core::drawing::{RaylibDraw, RaylibMode3DExt, RaylibDraw3D};
use raylib::color::Color;
use raylib::core::math::Vector3;

pub fn update_state(){

    let (mut rl, thread) = raylib::init()
        .size(1366, 768)
        .title("Limonka")
        .build();

    //// essential objects

    let cube_position = Vector3::zero();

    let mut camera = crate::utils::camera::topdown_camera::init_camera(&mut rl);

    //extra settings

    rl.set_target_fps(60);

    // main loop
    while !rl.window_should_close() {
        // input
        unsafe {
            input_map::handle_keys(&rl);
            input_map::handle_mouse(&rl);
        }

        //logic
        update_logic();

        // render
        rl.update_camera(&mut camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut d = d.begin_mode3D(&camera);

            d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
        }

        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }



    fn update_logic(){

    }
}
