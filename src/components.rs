use rltk::RGB;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
