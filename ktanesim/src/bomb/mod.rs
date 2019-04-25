use crate::prelude::*;
use arrayvec::ArrayVec;
use ktane_utils::edgework::Edgework;
use smallbitvec::SmallBitVec;
use std::collections::HashMap;
use std::sync::Arc;

mod context;
mod response;
mod starting;
mod timer;

pub use context::{get_bomb, need_bomb, no_bomb, running_in, update_presence, Bombs};
pub use response::{EventResponse, Render, RenderType};
pub use starting::cmd_run;
pub use timer::{Timer, TimerMode};

pub type BombRef = Arc<RwLock<Bomb>>;
pub struct Bomb {
    pub edgework: Edgework,
    pub rule_seed: u32,
    pub timer: Timer,
    pub modules: Vec<Box<dyn Module>>,
    // This field is not public to prevent the temptation to just flip the relevant bit in the
    // module code when solved. Some bookkeeping is necessary, and as such the proper method of
    // registering a solve is TODO
    solve_state: SmallBitVec,
    pub channel: ChannelId,
    pub defusers: HashMap<UserId, Defuser>,
}

pub type ModuleNumber = u8;
pub const MAX_CLAIMS: usize = 3;
pub const MAX_MODULES: ModuleNumber = 101;

/// Stores information about a user that has interacted with a bomb.
pub struct Defuser {
    pub last_view: ModuleNumber,
    pub claims: ArrayVec<[ModuleNumber; MAX_CLAIMS]>,
}

impl Bomb {
    fn get_last_view(&self, user: UserId) -> Result<ModuleNumber, ErrorMessage> {
        match self.defusers.get(&user) {
            Some(defuser) => Ok(defuser.last_view),
            None => Err((
                "You didn't view any modules".to_owned(),
                "The `!!` prefix sends the command to the module you have last viewed. \
                 However, you didn't look at any modules on this bomb yet. \
                 Try `!cvany` to claim a random module and start defusing."
                    .to_owned(),
            )),
        }
    }

    fn record_view(&mut self, user: UserId, num: ModuleNumber) {
        self.defusers
            .entry(user)
            .and_modify(|defuser| defuser.last_view = num)
            .or_insert_with(|| Defuser {
                last_view: num,
                claims: ArrayVec::new(),
            });
    }

    pub fn dispatch_module_command(
        &mut self,
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
            None => self.get_last_view(msg.author.id)?,
        };

        let cmd = params.next().unwrap_or("view");
        let response = match cmd {
            "view" => modcmd_view(self, num, params),
            _ => unimplemented!(),
        }?;

        if response.render.is_some() {
            self.record_view(msg.author.id, num);
        }

        response.resolve(ctx, msg, &*self.modules[num as usize]);
        Ok(())
    }
}

pub fn modcmd_view(
    bomb: &mut Bomb,
    num: ModuleNumber,
    mut params: Parameters<'_>,
) -> Result<EventResponse, ErrorMessage> {
    let module = &bomb.modules[num as usize];
    let light = if bomb.solve_state[num as usize] {
        SolveLight::Solved
    } else {
        SolveLight::Normal
    };

    let render = module.view(light);

    let message = if let Some(param) = params.next() {
        let mut ignored = vec![param];
        ignored.extend(params);
        Some((
            "Note: trailing parameters".to_owned(),
            MessageBuilder::new()
                .push(
                    "The `view` module command does not take any parameters. \
                     The following has been ignored: ",
                )
                .push_mono_safe(ignored.join(" "))
                .build(),
        ))
    } else {
        None
    };

    Ok(EventResponse {
        render: Some(render),
        message,
    })
}

// TODO: Ack and stuff
pub fn cmd_detonate(ctx: &Context, msg: &Message, params: Parameters<'_>) -> CommandResult {
    if ctx
        .data
        .write()
        .get_mut::<Bombs>()
        .unwrap()
        .remove(&msg.channel_id)
        .is_some()
    {
        update_presence(ctx);
        Ok(())
    } else {
        Err(no_bomb())
    }
}
