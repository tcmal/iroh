use iroh::*;

pub struct Rect {
    width: f32,
    height: f32,
}
impl Kind for Rect {
    fn kind_desc() -> KindDesc {
        KindDesc {}
    }
}

pub struct Circle {
    radius: f32
}
impl Kind for Circle {
    fn kind_desc() -> KindDesc {
        KindDesc {}
    }
}

pub struct MyFile {}
impl ObjectContainer for MyFile {
    type Kinds = ConsKinds<Rect, Circle>;
} 
