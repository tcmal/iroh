//! Lenses for our Rect type.

use super::Rect;
use iroh::lens::Lens;

#[derive(Debug, Clone)]
pub struct RectWidthLens;
impl Lens for RectWidthLens {
    type Source = Rect;
    type Target = f32;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        &source.width
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        &mut source.width
    }
}

#[derive(Debug, Clone)]
pub struct RectHeightLens;
impl Lens for RectHeightLens {
    type Source = Rect;
    type Target = f32;

    fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
        &source.height
    }

    fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
        &mut source.height
    }
}
