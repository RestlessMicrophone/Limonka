use raylib::input::*;
use raylib::RaylibHandle;


//boolean for inputs

    pub static mut Q: bool = false;
    pub static mut E: bool = false;

    pub unsafe fn handle_keys(handle : & RaylibHandle){

        if handle.is_key_pressed(raylib::consts::KeyboardKey::KEY_Q) {
            Q = true;
        }

        if handle.is_key_released(raylib::consts::KeyboardKey::KEY_Q) {
            Q = false;
        }

        if handle.is_key_pressed(raylib::consts::KeyboardKey::KEY_E) {
            E = true;
        }

        if handle.is_key_released(raylib::consts::KeyboardKey::KEY_E) {
            E = false;
        }

    }

    pub fn handle_mouse(handle : & RaylibHandle){


    }


