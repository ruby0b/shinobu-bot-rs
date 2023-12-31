use crate::consts::*;
use crate::prelude::*;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: &format!(
            "\
Type {COMMAND_PREFIX}help command for more info on a command.
You can edit your message to the bot and the bot will edit its response."
        ),
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
