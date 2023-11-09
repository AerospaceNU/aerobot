pub mod commands;

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, State, Error>;

pub struct State;