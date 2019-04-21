use crate::prelude::*;
use cairo::ImageSurface;
use lazy_static::lazy_static;
use serenity::utils::Colour;
use ktane_utils::edgework::{SerialNumber, Indicator};

pub fn cmd_edgework(
    ctx: &Context,
    msg: &Message,
    bomb: Arc<RwLock<Bomb>>,
    params: Parameters<'_>,
) -> CommandResult {
    bomb.read()
        .render_edgework()
        .resolve(ctx, msg.channel_id, |m, file| {
            m.embed(|e| e.color(Colour::DARK_GREEN).title("Edgework").image(file))
        });

    Ok(())
}

const RENDER_SLOT_DIM: (i32, i32) = (300, 150);
const RENDER_SLOTS: (i32, i32) = (3, 2);

impl crate::bomb::Bomb {
    pub fn render_edgework(&self) -> Render {
        let edgework = self.edgework.clone();

        Render::with(move || {
            let surface = ImageSurface::create(
                cairo::Format::ARgb32,
                RENDER_SLOT_DIM.0 * RENDER_SLOTS.0,
                RENDER_SLOT_DIM.1 * RENDER_SLOTS.1,
            )
            .unwrap();

            let ctx = CairoContext::new(&surface);
            let mut slots = (0..RENDER_SLOTS.0).flat_map(|x| (0..RENDER_SLOTS.1).map(move |y| (x, y)));
            let mut render = |widget: &dyn Widget| {
                let (x, y) = slots.next().unwrap();
                ctx.save();
                ctx.translate(
                    (RENDER_SLOT_DIM.0 * x + RENDER_SLOT_DIM.0 / 2) as f64,
                    (RENDER_SLOT_DIM.1 * y + RENDER_SLOT_DIM.1 / 2) as f64,
                );

                widget.render(&ctx);
                ctx.restore();
            };

            render(&edgework.serial_number);

            for _ in 0..edgework.aa_battery_pairs {
                let battery = Battery::DoubleAA;
                render(&battery);
            }

            for _ in 0..edgework.dcell_batteries {
                let battery = Battery::DCell;
                render(&battery);
            }

            for indicator in edgework.indicator_iter() {
                render(&indicator);
            }

            output_png(surface)
        })
    }
}

fn centered_texture(ctx: &CairoContext, texture: &SharedTexture) {
    ctx.set_source_surface(
        &texture.to_surface(),
        (-texture.width / 2) as f64,
        (-texture.height / 2) as f64,
    );
    ctx.paint();
}

fn centered_text(ctx: &CairoContext, text: &str, x: f64, y: f64) {
    // The "current point" used by show_text is the same as the one used for paths. This behavior
    // made only the first text object render properly. As a fix, `new_path` is called before each
    // text is rendered.
    ctx.new_path();
    let extents = ctx.text_extents(text);
    ctx.translate(x - extents.width / 2.0, y);
    ctx.show_text(text);
}

trait Widget {
    fn render(&self, ctx: &CairoContext);
}

impl Widget for SerialNumber {
    fn render(&self, ctx: &CairoContext) {
        lazy_static! {
            static ref TEXTURE: SharedTexture =
                SharedTexture::new(include_bytes!("Serial number.png"));
        }

        centered_texture(ctx, &TEXTURE);
        ctx.select_font_face(
            "Anonymous Pro",
            cairo::FontSlant::Normal,
            cairo::FontWeight::Bold,
        );
        ctx.set_font_size(65.0);
        ctx.set_source_rgb(0.0, 0.0, 0.0);
        centered_text(ctx, self.as_ref(), 0.0, 50.0);
    }
}

// The batteries are stored as a count, so we need a "dummy" struct to implement Widget on in order
// to take advantage of the same pipeline as the other widgets
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Battery {
    DCell,
    // Yes - they come in pairs.
    DoubleAA,
}

impl Widget for Battery {
    fn render(&self, ctx: &CairoContext) {
        lazy_static! {
            static ref TEXTURE_AA: SharedTexture =
                SharedTexture::new(include_bytes!("BatteryAA.png"));
            static ref TEXTURE_D: SharedTexture =
                SharedTexture::new(include_bytes!("BatteryD.png"));
        }

        match self {
            Battery::DoubleAA => centered_texture(ctx, &TEXTURE_AA),
            Battery::DCell => centered_texture(ctx, &TEXTURE_D),
        }
    }
}

impl Widget for Indicator {
    fn render(&self, ctx: &CairoContext) {
        lazy_static! {
            static ref TEXTURE_UNLIT: SharedTexture =
                SharedTexture::new(include_bytes!("UnlitIndicator.png"));
            static ref TEXTURE_LIT: SharedTexture =
                SharedTexture::new(include_bytes!("LitIndicator.png"));
        }

        let texture: &SharedTexture = match self.lit {
            true => &TEXTURE_LIT,
            false => &TEXTURE_UNLIT,
        };

        centered_texture(ctx, texture);
        ctx.select_font_face(
            "Ostrich Sans",
            cairo::FontSlant::Normal,
            cairo::FontWeight::Bold,
        );
        ctx.set_font_size(60.0);
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        centered_text(ctx, self.code.into(), 35.0, 20.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_comparison(widget: impl Widget, mut expected: &[u8], actual: &str) {
        let mut surface = ImageSurface::create(
            cairo::Format::ARgb32,
            RENDER_SLOT_DIM.0,
            RENDER_SLOT_DIM.1,
        ).expect("Cannot create test surface");
        let ctx = CairoContext::new(&surface);
        ctx.translate((RENDER_SLOT_DIM.0 / 2) as f64, (RENDER_SLOT_DIM.1 / 2) as f64);
        widget.render(&ctx);
        drop(ctx);

        let mut expected = ImageSurface::create_from_png(
            &mut expected
        ).expect("Cannot parse expected result");

        if *expected.get_data().unwrap() != *surface.get_data().unwrap() {
            surface.write_to_png(
                &mut std::fs::File::create(actual)
                .expect("Cannot create result file")
            ).expect("Cannot write to result file");
            panic!("Pixels don't match");
        }
    }

    #[test]
    fn serial_number_font() {
        let serial: SerialNumber = "TW8LF0".parse().unwrap();
        render_comparison(serial, include_bytes!("Serial-expected.png"), "Serial-actual.png");
    }

    #[test]
    fn indicator_font() {
        let indicator = Indicator { code: ktane_utils::edgework::IndicatorCode::BOB, lit: true };
        render_comparison(indicator, include_bytes!("Indicator-expected.png"), "Indicator-actual.png");
    }
}
