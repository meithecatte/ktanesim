use crate::Bomb;
use std::boxed::FnBox;
use std::sync::MutexGuard;
use typemap::ShareMap;
use serenity::model::id::UserId;

pub const MODULE_SIZE: i32 = 348;
pub type Render = Box<dyn FnBox() -> (Vec<u8>, RenderType)>;
pub type ModuleNew = fn(bomb: &mut Bomb, rule_cache: MutexGuard<ShareMap>) -> Box<dyn Module>;

use strum_macros::Display;
#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum RenderType {
    PNG,
    GIF,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SolveLight {
    Normal,
    Solved,
    Strike,
}

use cairo::{Context, ImageSurface};
/// Return an `ImageSurface` with a blank module drawn on it, along with a `Context` that can be used
/// to draw any further graphics.
pub fn module_canvas(status: SolveLight) -> (ImageSurface, Context) {
    let surface = ImageSurface::create(cairo::Format::ARgb32, MODULE_SIZE, MODULE_SIZE).unwrap();
    let ctx = Context::new(&surface);

    ctx.set_line_join(cairo::LineJoin::Round);

    ctx.rectangle(5.0, 5.0, 338.0, 338.0);
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.fill_preserve();
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.stroke();

    ctx.arc(298.0, 40.5, 15.0, 0.0, 2.0 * std::f64::consts::PI);

    use SolveLight::*;
    match status {
        Normal => ctx.set_source_rgb(1.0, 1.0, 1.0),
        Solved => ctx.set_source_rgb(0.0, 1.0, 0.0),
        Strike => ctx.set_source_rgb(1.0, 0.0, 0.0),
    }

    ctx.fill_preserve();
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.stroke();

    (surface, ctx)
}

/// Given an `ImageSurface`, return what an implementation of `Render` should.
pub fn output_png(surface: ImageSurface) -> (Vec<u8>, RenderType) {
    let mut png = std::io::Cursor::new(vec![]);
    surface.write_to_png(&mut png).unwrap();
    (png.into_inner(), RenderType::PNG)
}

pub struct EventResponse {
    render: Option<Render>,
    message: Option<String>,
}

pub trait Module {
    fn handle_command(&mut self, bomb: &mut Bomb, user: UserId, command: &str) -> EventResponse;
    fn view(&self) -> Render;
}

use phf_macros::phf_map;
static MODULES: phf::Map<&'static str, ModuleNew> = phf_map! {
    "wires" => wires::init,
    "simplewires" => wires::init,
};

pub mod wires;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_type_is_lowercase() {
        assert_eq!(RenderType::PNG.to_string(), "png");
    }
}
