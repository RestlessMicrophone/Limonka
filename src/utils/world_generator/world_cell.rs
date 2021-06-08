use raylib::math::Vector3;

pub struct MapCell {
    pub(crate) pos_x: i32,
    pub(crate) pos_y: i32
}

impl MapCell{

    pub fn get3DVector(&self) -> Vector3{

        let x = self.pos_x.clone();
        let y = self.pos_y.clone();

        let toReturn = Vector3{
            x: x as f32,
            y: 0.0,
            z: y as f32
        };
        toReturn
    }

    pub fn add_vector(&mut self, x: usize, y: usize){
        let addX = x.clone() as i32;
        let addY = y.clone() as i32;

        self.pos_x += addX;
        self.pos_y += addY;
    }
}

pub fn return_default_cell(pos_x: i32, pos_y: i32) -> MapCell {
    let to_return = MapCell {
        pos_x,
        pos_y
    };
    to_return
}