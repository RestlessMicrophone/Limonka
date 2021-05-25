//currently specifically made for the current system

use raylib::{RaylibHandle, RaylibThread};
use raylib::core::models::{RaylibModel, RaylibMaterial, Model};

pub struct handler <'a> {
    pub(crate) handle:  &'a mut RaylibHandle,
    pub(crate) thread: &'a  mut RaylibThread
}

impl<'a> handler <'a>{
    pub fn return_model(&mut self, path_model: &String, path_texture: &String) -> Model {
        let mut model = self.handle
            .load_model(self.thread, path_model)
            .unwrap(); // Load the animated model mesh and basic data
        let texture = self.handle
            .load_texture(self.thread, path_texture)
            .unwrap(); // Load model texture and set material
        & mut model.materials_mut()[0]
            .set_material_texture(raylib::consts::MaterialMapType::MAP_ALBEDO, &texture);

        model
    }
}