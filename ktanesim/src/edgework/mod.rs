use crate::prelude::*;
use cairo::ImageSurface;
use serenity::utils::Colour;
use lazy_static::lazy_static;

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

lazy_static! {
    static ref SERIAL_NUMBER_TEXTURE: SharedTexture =
        SharedTexture::new(include_bytes!("Serial number.png"));
}

const RENDER_SLOT_DIM: (i32, i32) = (269, 136);
const RENDER_SLOTS: (i32, i32) = (3, 2);

impl crate::bomb::Bomb {
    pub fn render_edgework(&self) -> Render {
        Render::with(|| {
            let surface = ImageSurface::create(
                cairo::Format::ARgb32,
                RENDER_SLOT_DIM.0 * RENDER_SLOTS.0,
                RENDER_SLOT_DIM.1 * RENDER_SLOTS.1,
            ).unwrap();

            let ctx = CairoContext::new(&surface);

            let texture = SERIAL_NUMBER_TEXTURE.to_surface();

            for x in 0..RENDER_SLOTS.0 {
                for y in 0..RENDER_SLOTS.1 {
                    ctx.save();
                    ctx.translate(
                        (RENDER_SLOT_DIM.0 * x) as f64,
                        (RENDER_SLOT_DIM.1 * y) as f64,
                    );

                    ctx.set_source_surface(&texture, 0.0, 0.0);
                    ctx.paint();

                    ctx.restore();
                }
            }

            output_png(surface)
        })
    }
}
