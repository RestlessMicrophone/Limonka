use raylib::prelude::*;


    pub fn init_camera(handle: &mut RaylibHandle) -> Camera3D{
        let mut camera = Camera3D::perspective(
            rvec3(10.0, 10.0, 10.0), // Camera position
            rvec3(0.0, 0.0, 0.0),    // Camera looking at point
            rvec3(0.0, 1.0, 0.0),    // Camera up vector (rotation towards target)
            45.0,                    // Camera field-of-view Y
        );

        handle.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FREE); // Set a free camera mode

        camera
        }









