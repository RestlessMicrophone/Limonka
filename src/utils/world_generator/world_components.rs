// here we list all the components world will be using

use specs::{Component, VecStorage};

impl Component for Position {
    type Storage = VecStorage<Self>;
}


#[derive(Debug)]
pub struct Velocity {
    pub(crate)  x: f32,
    pub(crate)  y: f32,
}

#[derive(Debug)]
pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}



