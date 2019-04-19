use crate::prelude::*;
use serenity::utils::Colour;

pub fn cmd_edgework(
    ctx: &Context,
    msg: &Message,
    bomb: Arc<RwLock<Bomb>>,
    params: Parameters<'_>,
) -> CommandResult {
    bomb.read().render_edgework().resolve(ctx, msg.channel_id, |m, file| {
        m.embed(|e| e.color(Colour::DARK_GREEN).title("Edgework").image(file))
    });

    Ok(())
}

impl crate::bomb::Bomb {
    pub fn render_edgework(&self) -> Render {
        Render::with(|| {
            (include_bytes!("Serial number.png").to_vec(), RenderType::PNG)
        })
    }
}
