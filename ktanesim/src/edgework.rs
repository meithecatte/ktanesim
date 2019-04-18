use crate::prelude::*;

pub fn cmd_edgework(
    ctx: &Context,
    msg: &Message,
    bomb: Arc<RwLock<Bomb>>,
    params: Parameters<'_>,
) -> CommandResult {
    unimplemented!();
}

impl crate::bomb::Bomb {
    pub fn render_edgework(&self) -> Render {
        unimplemented!();
    }
}
