use raylib::math::Vector3;

pub struct WalkCell {
    pub(crate) pos_x: i64,
    pub(crate) pos_y: i64
}

impl WalkCell {

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

    pub fn add_vector(&mut self, x: i64, y: i64){
        let addX = x.clone() as i64;
        let addY = y.clone() as i64;

        self.pos_x += addX;
        self.pos_y += addY;
    }
}

pub fn return_default_cell(pos_x: i64, pos_y: i64) -> WalkCell {
    let to_return = WalkCell {
        pos_x,
        pos_y
    };
    to_return
}