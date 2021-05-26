
pub struct MapCell {
    pub(crate) pos_x: i32,
    pub(crate) pos_y: i32
}

pub fn return_default_cell(pos_x: i32, pos_y: i32) -> MapCell {
    let to_return = MapCell {
        pos_x,
        pos_y
    };
    to_return
}