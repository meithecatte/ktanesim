use crate::prelude::*;
use strum_macros::Display;

pub struct EventResponse {
    pub render: Option<Render>,
    pub message: Option<(String, String)>,
}

impl EventResponse {
    /// Turn the EventResponse into a message and send it.
    pub fn resolve(self, ctx: &Context, msg: &Message, module: &dyn Module) {
        match self {
            EventResponse {
                render: Some(render),
                message: None,
            } => {
                render.resolve(ctx, msg.channel_id, |m, file| {
                    m.embed(|e| e.title(module.module_name()).image(file))
                });
            }
            _ => unimplemented!(),
        }
    }
}

pub struct Render(pub Box<dyn FnOnce() -> (Vec<u8>, RenderType)>);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum RenderType {
    PNG,
    GIF,
}

use serenity::builder::CreateMessage;
impl Render {
    /// Helper method for simpler creation of Render objects
    pub fn with(f: impl FnOnce() -> (Vec<u8>, RenderType) + 'static) -> Render {
        Render(Box::new(f))
    }

    pub fn resolve<F>(self, ctx: &Context, channel_id: ChannelId, f: F)
    where
        for<'b> F: FnOnce(&'b mut CreateMessage<'b>, &str) -> &'b mut CreateMessage<'b>,
    {
        let (data, extension) = (self.0)();
        let filename = format!("f.{}", extension);
        if let Err(why) = channel_id.send_files(
            &ctx.http,
            std::iter::once((&data[..], &filename[..])),
            |m| f(m, &format!("attachment://{}", filename)),
        ) {
            error!("Couldn't send message with attachment: {:?}", why);
        }
    }
}
