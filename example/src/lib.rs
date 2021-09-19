#![crate_type = "dylib"]

use iroh::*;
use iroh_editor::ffi::AddonPanes;

mod panes;
use panes::MyPanes;

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
    radius: f32,
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

#[no_mangle]
extern "C" fn get_addon_panes() -> Box<dyn AddonPanes> {
    Box::new(MyPanes)
}
