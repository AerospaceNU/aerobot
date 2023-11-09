use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// info about a voice channel
#[poise::command(slash_command)]
pub async fn voiceinfo(
    ctx: Context<'_>,
    #[description = "Information about a server voice channel"]
    #[channel_types("Voice")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let response = format!(
        "\
**Name**: {}
**Bitrate**: {}
**User limit**: {}
**RTC region**: {}
**Video quality mode**: {:?}",
        channel.name,
        channel.bitrate.unwrap_or_default(),
        channel.user_limit.unwrap_or_default(),
        channel.rtc_region.unwrap_or_default(),
        channel
            .video_quality_mode
            .unwrap_or(serenity::VideoQualityMode::Unknown)
    );

    ctx.say(response).await?;
    Ok(())
}

/// basically bash echo
#[poise::command(prefix_command, slash_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[rest]
    #[description = "Text to say"]
    msg: String,
) -> Result<(), Error> {
    ctx.say(msg).await?;
    Ok(())
}