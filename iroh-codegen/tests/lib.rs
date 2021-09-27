use iroh::lens::Lens;
use iroh_codegen::Lens;

#[derive(Lens)]
pub struct Rect {
    width: f32,
    height: f32,
}

#[derive(Lens)]
pub struct Vec2(f32, f32);

#[test]
fn test_struct_lenses() {
    let r = Rect {
        width: 1.0,
        height: 2.0,
    };

    assert_eq!(1.0, *RectWidthLens::get(&r));
    assert_eq!(2.0, *RectHeightLens::get(&r));
}

#[test]
fn test_tuple_lenses() {
    let v = Vec2(1.0, 2.0);

    assert_eq!(1.0, *Vec20Lens::get(&v));
    assert_eq!(2.0, *Vec21Lens::get(&v));
}
