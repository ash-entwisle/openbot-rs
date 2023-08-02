
use crate::{Context, Error};
use poise::CreateReply;
use poise::serenity_prelude::CreateEmbed;

/// A command to test the bot's latency
/// 
/// > This command gets the latency of the bot in milliseconds
/// 
/// **Formats:** 
/// ```
/// - /ping
/// ```
#[poise::command(prefix_command, slash_command)]
pub async fn ping(
    ctx: Context<'_>,

) -> Result<(), Error> {

    let latency: i64 = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)?
        .as_millis() as i64 
        -ctx.created_at()
        .timestamp_millis();
    
    let colour: &String = ctx.data()
        .config
        .embed
        .color
        .as_ref()
        .unwrap();
        
    ctx.send(|r: &mut CreateReply<'_>| {
        r.embed(|e: &mut CreateEmbed| {e
            .title("Pong!")
            .description(format!("latency: `{}ms`", latency))
            .colour(u64::from_str_radix(&colour[1..], 16).unwrap_or(0))
        });
        r.ephemeral(true)
    }).await?;

    Ok(())
}
