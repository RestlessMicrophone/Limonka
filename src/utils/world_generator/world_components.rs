// here we list all the components world will be using

use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct position3D{
    pub(crate) posX: f64,
    pub(crate) posY: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct position{
    posX: i32,
    posY: i32
}



