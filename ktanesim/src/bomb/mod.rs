use crate::prelude::*;
use crate::utils::{ranged_int_parse, RangedIntError};
use arrayvec::ArrayVec;
use serenity::utils::Colour;
use serenity::http::raw::Http;
use std::collections::HashMap;
use std::sync::Arc;

mod context;
mod response;
pub mod starting;
mod timer;

pub use context::{end_bomb, get_bomb, need_bomb, running_in, update_presence};
pub use response::{EventResponse, Render, RenderType};
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
    // TODO: use upgradable read
    // TODO: add !! hint to no such command errors
    pub fn dispatch_module_command(
        &mut self,
        handler: &Handler,
        ctx: &Context,
        msg: &Message,
        num: Option<&str>,
        mut params: Parameters<'_>,
    ) -> Result<(), ErrorMessage> {
        let num: ModuleNumber = match num {
            Some(num) => match ranged_int_parse(num, self.data.module_count) {
                Ok(0) => return Err(ErrorMessage::ModuleNumberZero),
                Ok(n) => n - 1,
                Err(RangedIntError::TooLarge) => {
                    return Err(ErrorMessage::ModuleNumberTooLarge {
                        num: num.to_owned(),
                        max: self.data.module_count
                    });
                }
                Err(_) => unreachable!(), // dispatch ensures the string is alphanumeric
            },
            None => self.data.get_last_view(msg.author.id).ok_or(ErrorMessage::NoLastView)?,
        };

        let response = match params.next() {
            Some("view") | None => modcmd_view(self, num, params),
            Some(other) => {
                let module = &mut self.modules[num as usize];

                if module.state().solved() {
                    return Err(ErrorMessage::ModuleAlreadySolved(num));
                }

                // TODO: handle claimed modules
                module.handle_command(&mut self.data, msg.author.id, other, params)
            }
        }?;

        if response.render.is_some() {
            self.data.record_view(msg.author.id, num);
        }

        // TODO: drop lock before rendering
        response.resolve(ctx, msg, self, &*self.modules[num as usize]);

        if self.data.solved_count == self.data.solvable_count {
            info!("Solved all modules");
            let http = Arc::clone(&ctx.http);
            crate::bomb::end_bomb(handler, &mut self.data, move |bomb| {
                crate::utils::send_message(&http, bomb.channel, |m| {
                    m.embed(|e| {
                        e.color(Colour::DARK_GREEN);
                        e.title("Bomb defused \u{1f389}");
                        e.description(format!(
                            "After {} strikes, the bomb has been **defused**",
                            bomb.timer.strikes(),
                        ));
                        
                        e.field(bomb.timer.field_name(), &bomb.timer, true)
                    })
                });
            });
        }

        if self.data.timer.strike_explosion() {
            info!("Strikes ran out");
            self.explode(handler, Arc::clone(&ctx.http), self.modules[num as usize].name());
        }

        Ok(())
    }

    pub fn explode(
        &mut self,
        handler: &Handler,
        http: Arc<Http>,
        cause: &'static str,
    ) {
        crate::bomb::end_bomb(handler, &mut self.data, move |bomb| {
            crate::utils::send_message(&http, bomb.channel, |m| {
                m.embed(|e| {
                    e.color(Colour::RED);
                    e.title("Bomb exploded \u{1f4a5}");
                    e.field(bomb.timer.field_name(), &bomb.timer, true);
                    e.field("Cause of explosion", cause, true)
                })
            });
        });
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

// TODO: Majority agreement
pub fn cmd_detonate(
    handler: &Handler,
    ctx: &Context,
    msg: &Message,
    bomb: BombRef,
    params: Parameters<'_>,
) -> CommandResult {
    bomb.write().explode(handler, Arc::clone(&ctx.http), "Humans got bored");
    Ok(())
}
