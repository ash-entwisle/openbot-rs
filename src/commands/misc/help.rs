use poise::serenity_prelude::Timestamp;
use chrono;
use crate::{Context, Error};

async fn help_some(ctx: &Context<'_>, command_name: &String) -> Result<String, Error> {

    // match command to ctx.framework().options().commands
    let command = ctx
        .framework()
        .options()
        .commands
        .iter()
        .find(|command| {
            if command.name.eq_ignore_ascii_case(&command_name) { return true; } 
            if let Some(context_menu_name) = command.context_menu_name {
                if context_menu_name.eq_ignore_ascii_case(&command_name) { return true; }
            }
            false
        });

    let help_data = if let Some(command) = command {
        match command.help_text {
            Some(f) => f(),
            None => command
                .description
                .as_ref()
                .unwrap_or(&"No description available".to_string())
                .to_owned(),
        }
    } else {
        format!("Command `{}` not found", command_name)
    };


    Ok(help_data)
}

async fn help_none(ctx: &Context<'_>) -> Result<String, Error> {

    let config = ctx.data().config.clone();

    let lines: Vec<String> = vec![
        format!("To get help, please visit the docs at {} or run `/help [command name]` to get more information on a spesific command.  ", config.info.docs.unwrap()),
        format!("If you are still stuck, feel free to join the support server at {} and ask a question in the *Help* forum.  ", config.info.server.unwrap()),
    ];

    // // commented out bc there are probs gonna be too many commands for this to be useful
    // let commands = ctx
    //     .framework()
    //     .options()
    //     .commands
    //     .iter();
    // 
    // let mut help_data: Vec<String> = commands
    //     .map(|command| {
    //         format!("**/{}** \n > {} \n", command.name, command.description.as_ref().unwrap_or(&"No description available".to_string()))
    //     })
    //     .collect();
    //
    // help_data.sort();

    Ok(lines.join("\n\n"))
}


/// Show Help Text for a Command or All Commands
///
/// > This command shows help text for a specific command or for all commands if no command is specified.
/// 
/// **Formats: **
/// ```
/// - /help [command name]
/// - /help 
/// ```
#[poise::command(
    rename = "help",
    slash_command,
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {

    let config = ctx.data().config.clone();

    let command_name: String = match &command {
        Some(command) => command.to_string(),
        None => "".to_string(),
    };

    let title: String = match &command {
        Some(_) => format!("**Command: `/{}`**", &command_name),
        None => "".to_string(),
    };

    let description: String = match &command {
        Some(_) => help_some(&ctx, &command_name).await?,
        None => help_none(&ctx).await?,
    };

    let colour: &String = ctx.data()
        .config
        .embed
        .color
        .as_ref()
        .unwrap();

    ctx.send(|r | r
        .embed(|e| e
            .title(title)
            .url(ctx.data().config.info.docs.clone().unwrap() + "/commands/" + &command_name)
            .description(description)
            .colour(u64::from_str_radix(&colour[1..], 16).unwrap_or(0))
            .footer(|f| f.icon_url(config.info.pfp.clone().unwrap()))
            .timestamp(Timestamp::from(chrono::Utc::now()))
        )
        .ephemeral(true)
    ).await?;

    Ok(())
}
