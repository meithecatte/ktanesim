use crate::prelude::*;
use cairo::ImageSurface;
use lazy_static::lazy_static;
use serenity::utils::Colour;

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

            output_png(surface)
        })
    }
}

fn draw_centered_texture(ctx: &CairoContext, texture: &SharedTexture) {
    ctx.set_source_surface(
        &texture.to_surface(),
        (-texture.width / 2) as f64,
        (-texture.height / 2) as f64,
    );
    ctx.paint();
}

trait Widget {
    fn render(&self, ctx: &CairoContext);
}

impl Widget for ktane_utils::edgework::SerialNumber {
    fn render(&self, ctx: &CairoContext) {
        lazy_static! {
            static ref TEXTURE: SharedTexture =
                SharedTexture::new(include_bytes!("Serial number.png"));
        }

        draw_centered_texture(ctx, &TEXTURE);
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

        let texture: &SharedTexture = match self {
            Battery::DoubleAA => &TEXTURE_AA,
            Battery::DCell => &TEXTURE_D,
        };

        draw_centered_texture(ctx, texture);
    }
}
