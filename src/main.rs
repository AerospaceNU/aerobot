use poise::serenity_prelude::{self as serenity};
use std::{env::var, time::Duration};
use dotenv;
use aerobot::{State, Context, Error};
use aerobot::commands::*;

/// Show this help menu
#[poise::command(track_edits, slash_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "\n
A bot for the AeroNU Discord server",
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Registers or unregisters application commands in this guild or globally
#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    // this command allows us to run [prefix]register to register application commands
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

/// Error handler for the function
async fn on_error(error: poise::FrameworkError<'_, State, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var, see README for more information.");

//    env_logger::init();

    let options = poise::FrameworkOptions {

        // The commands to register for this bot
        commands: vec![
            help(),
            register(),
            general::voiceinfo(),
            general::echo(),
        ],

        // The options for the prefix to normal commands in this bot
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            ..Default::default()
        },

        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),

        /// This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },

        /// This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },

        /// Placeholder command check
        command_check: Some(|ctx: Context| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),

        // This code will handle events and event types
        event_handler: |ctx, event, _framework, _state| {
            Box::pin(async move {
                // Specific Event Handlers
                match event {
                    poise::Event::ReactionAdd { add_reaction: reaction } => {
                        if let serenity::ReactionType::Unicode(emoji) = &reaction.emoji {
                            if emoji == "📌" {
                                let msg = reaction.message(&ctx).await?;
                                msg.pin(&ctx).await?;
                            }
                        }
                    },
                    poise::Event::ReactionRemove { removed_reaction: reaction } => {
                        if let serenity::ReactionType::Unicode(emoji) = &reaction.emoji {
                            if emoji == "📌" {
                                let msg = reaction.message(&ctx).await?;
                                msg.unpin(&ctx).await?;
                            }
                        }
                    },
                    // refactor this handline out of here and into lib, add join/leave messages
                    _ => {},
                }
                Ok(())
            }
        )},

        /// Enforce command checks even for owners (enforced by default)
        /// Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        ..Default::default()
    };

    poise::Framework::builder()
        .token(&token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {

                // Register all commands globally
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // initialize state here
                Ok(State {})
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run().await.unwrap();
}