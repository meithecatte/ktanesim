use super::env;
use serenity::model::prelude::*;
use std::collections::HashSet;
use std::error::Error;

pub struct Config {
    allow_dm: bool,
    bot_owner: UserId,
    allowed_channels: HashSet<ChannelId>,
}

impl Config {
    pub fn parse() -> Result<Config, Box<dyn Error>> {
        Ok(Config {
            allow_dm: env("ALLOW_DM").parse()?,
            bot_owner: UserId(env("BOT_OWNER").parse()?),
            allowed_channels: env("ALLOWED_CHANNELS")
                .split(',')
                .map(|s| s.parse().map(|id| ChannelId(id)))
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn should_ignore_message(&self, msg: &Message) -> bool {
        if msg.author.bot {
            return true;
        }

        if msg.guild_id.is_some() {
            !self.allowed_channels.contains(&msg.channel_id)
        } else {
            !self.allow_dm && msg.author.id != self.bot_owner
        }
    }
}
