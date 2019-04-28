use crate::prelude::*;
use arrayvec::ArrayVec;
use serenity::utils::Colour;
use std::collections::HashMap;
use std::sync::Arc;

mod context;
mod response;
mod starting;
mod timer;

pub use context::{end_bomb, get_bomb, need_bomb, no_bomb, running_in, update_presence};
pub use response::{EventResponse, Render, RenderType};
pub use starting::cmd_run;
pub use timer::{Timer, TimerMode};

pub type BombRef = Arc<RwLock<Bomb>>;
/// Because of borrowing concerns, bombs consist of [`BombData`] and a [`Vec`] of [`Module`]s.
pub struct Bomb {
    pub modules: Vec<Box<dyn Module>>,
    pub data: BombData,
}

pub struct BombData {
    pub edgework: Edgework,
    pub rule_seed: u32,
    pub module_count: ModuleNumber,
    /// Stores the number of non-needy modules on this bomb.
    pub solvable_count: ModuleNumber,
    pub solved_count: ModuleNumber,
    pub timer: Timer,
    pub channel: ChannelId,
    pub defusers: HashMap<UserId, Defuser>,
    drop_callback: Option<Box<dyn FnOnce(&mut BombData) + Send + Sync>>,
}

pub type ModuleNumber = u8;
pub const MAX_CLAIMS: usize = 3;
pub const MAX_MODULES: ModuleNumber = 101;

/// Stores information about a user that has interacted with a bomb.
pub struct Defuser {
    pub last_view: ModuleNumber,
    pub claims: ArrayVec<[ModuleNumber; MAX_CLAIMS]>,
}

impl Drop for BombData {
    fn drop(&mut self) {
        if let Some(callback) = self.drop_callback.take() {
            callback(self);
        } else {
            warn!(
                "Bomb dropped without callback in channel {:?}",
                self.channel
            )
        }
    }
}

impl BombData {
    pub fn get_last_view(&self, user: UserId) -> Option<ModuleNumber> {
        self.defusers.get(&user).map(|defuser| defuser.last_view)
    }

    pub fn need_last_view(&self, user: UserId) -> Result<ModuleNumber, ErrorMessage> {
        self.get_last_view(user).ok_or_else(|| {
            (
                "You didn't view any modules".to_owned(),
                "The `!!` prefix sends the command to the module you have last viewed. \
                 However, you didn't look at any modules on this bomb yet. \
                 Try `!cvany` to claim a random module and start defusing."
                    .to_owned(),
            )
        })
    }

    pub fn record_view(&mut self, user: UserId, num: ModuleNumber) {
        self.defusers
            .entry(user)
            .and_modify(|defuser| defuser.last_view = num)
            .or_insert_with(|| Defuser {
                last_view: num,
                claims: ArrayVec::new(),
            });
    }
}

impl Bomb {
    pub fn dispatch_module_command(
        &mut self,
        handler: &Handler,
        ctx: &Context,
        msg: &Message,
        num: Option<&str>,
        mut params: Parameters<'_>,
    ) -> Result<(), ErrorMessage> {
        let num: ModuleNumber = match num {
            Some(num) => match num.parse() {
                Ok(num) if (1..=self.modules.len() as ModuleNumber).contains(&num) => num - 1,
                // The string is all-numeric because of the code in commands.rs, so it's just too
                // large.
                _ => {
                    if num == "0" {
                        return Err((
                            "Module numbers start at 1".to_owned(),
                            "`0` is not a valid module number. The numbering starts at 1."
                                .to_owned(),
                        ));
                    } else if self.modules.len() == 1 {
                        return Err((
                            "Module number too large".to_owned(),
                            format!(
                                "`{}` is not a valid module number. \
                                 There is only one module on this bomb.",
                                num,
                            ),
                        ));
                    } else {
                        return Err((
                            "Module number too large".to_owned(),
                            format!(
                                "`{}` is not a valid module number. \
                                 There are only {} modules on this bomb.",
                                num,
                                self.modules.len()
                            ),
                        ));
                    }
                }
            },
            None => self.data.need_last_view(msg.author.id)?,
        };

        let response = match params.next() {
            Some("view") | None => modcmd_view(self, num, params),
            Some(other) => {
                let module = &mut self.modules[num as usize];

                if module.state().solved() {
                    return Err((
                        "Module already solved".to_owned(),
                        format!(
                            "Module #{} has already been solved. \
                             Try `!cvany` to claim a new module.",
                            num + 1
                        ),
                    ));
                }

                // TODO: handle claimed modules
                module.handle_command(&mut self.data, msg.author.id, other, params)
            }
        }?;

        if response.render.is_some() {
            self.data.record_view(msg.author.id, num);
        }

        response.resolve(ctx, msg, self, &*self.modules[num as usize]);

        if self.data.solved_count == self.data.solvable_count {
            info!("Solved all modules");
            let http = Arc::clone(&ctx.http);
            crate::bomb::end_bomb(handler, ctx, &mut self.data, move |bomb| {
                crate::utils::send_message(&http, bomb.channel, |m| {
                    m.embed(|e| {
                        e.color(Colour::DARK_GREEN);
                        e.title("Bomb defused \u{1f389}");
                        e.description(format!(
                            "After {} strikes, the bomb has been **defused**, \
                             with {} on the timer.",
                            bomb.timer.strikes(),
                            bomb.timer
                        ))
                    })
                });
            });
        }

        Ok(())
    }
}

pub fn modcmd_view(
    bomb: &mut Bomb,
    num: ModuleNumber,
    params: Parameters<'_>,
) -> Result<EventResponse, ErrorMessage> {
    let module = &bomb.modules[num as usize];
    let light = if module.state().solved() {
        SolveLight::Solved
    } else {
        SolveLight::Normal
    };

    let render = module.view(light);
    let message = crate::utils::trailing_parameters(params).map(|params| {
        (
            "Note: trailing parameters".to_owned(),
            MessageBuilder::new()
                .push(
                    "The `view` module command does not take any parameters. \
                     The following has been ignored: ",
                )
                .push_mono_safe(params)
                .build(),
        )
    });

    Ok(EventResponse {
        render: Some(render),
        message,
    })
}

// TODO: Ack and stuff
// TODO: use end_bomb
pub fn cmd_detonate(
    handler: &Handler,
    ctx: &Context,
    msg: &Message,
    params: Parameters<'_>,
) -> CommandResult {
    if handler.bombs.write().remove(&msg.channel_id).is_some() {
        update_presence(handler, ctx);
        Ok(())
    } else {
        Err(no_bomb())
    }
}
