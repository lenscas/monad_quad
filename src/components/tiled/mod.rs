use std::{borrow::Cow, cell::RefCell, rc::Rc};

use macroquad::{miniquad::gl::glFlush, prelude::Rect, window::get_internal_gl};
use macroquad_tiled::Map;

use crate::Component;

#[derive(Clone)]
pub struct TiledProperties<'a> {
    pub map: Rc<RefCell<Map>>,
    pub layer: Cow<'a, str>,
    pub dest: Rect,
    pub source: Option<Rect>,
}

pub struct Tiled;

impl<'a> Component<&TiledProperties<'a>, &mut TiledProperties<'a>> for Tiled {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }
    fn render(&self, _: &super::Context, props: &TiledProperties<'a>) {
        props
            .map
            .borrow()
            .draw_tiles(&props.layer, props.dest, props.source);
        let mut gl = unsafe { get_internal_gl() };
        gl.flush()
    }
}
