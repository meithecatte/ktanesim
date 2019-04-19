use crate::prelude::*;

/// Texture data shareable between threads.
///
/// Cairo's ImageSurface is !Send and !Sync, so it's not possible to keep textures as persistent
/// image surfaces. Nevertheless, we want to avoid decoding the PNG data every time the texture
/// needs to be used. Therefore, this structure stores the raw pixel data and constructs cairo
/// surfaces on demand. Note that it's still necessary to clone the pixel data vector due to the
/// lack of distinction between a surface used as a source vs. a target in the API.
pub struct SharedTexture {
    format: cairo::Format,
    // Yes, signed integer. That's just how the API works. Yes, I know that's ugly.
    pub width: i32,
    pub height: i32,
    stride: i32,
    data: Vec<u8>,
}

impl SharedTexture {
    pub fn new(mut png: &[u8]) -> SharedTexture {
        let mut surface = ImageSurface::create_from_png(&mut png).unwrap();
        let data = surface.get_data().unwrap().to_vec();

        SharedTexture {
            format: surface.get_format(),
            width: surface.get_width(),
            height: surface.get_height(),
            stride: surface.get_stride(),
            data,
        }
    }

    pub fn to_surface(&self) -> ImageSurface {
        ImageSurface::create_for_data(
            self.data.clone(),
            self.format,
            self.width,
            self.height,
            self.stride,
        )
        .unwrap()
    }
}
