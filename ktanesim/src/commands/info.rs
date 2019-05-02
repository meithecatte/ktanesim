use crate::prelude::*;

pub fn cmd_source(
    handler: &Handler,
    ctx: &Context,
    msg: &Message,
    params: Parameters<'_>,
) -> CommandResult {
    crate::utils::send_message(&ctx.http, msg.channel_id, |m| {
        m.embed(|e| {
            e.title("Source code");
            e.description(format!("Click [here]({}) to see the source code.", handler.config.source_url));
            e.field("Build info", format!("```{}```", crate::TESTAMENT), false);

            if let Some(params) = crate::utils::trailing_parameters(params) {
                e.field("Note: trailing parameters",
                    MessageBuilder::new()
                        .push("The `source` command does not take any parameters. \
                               The following has been ignored: ")
                        .push_mono_safe(params)
                        .build(),
                    false
                );
            }

            e
        })
    });

    Ok(())
}
