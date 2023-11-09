# How to contribute
- Anyone in AeroNU can make a pull request to this bot
- If you want to make any super large changes or are unsure, please contact @sylkos on discord

## How to add a command
Go to `commands/general.rs` and copy the echo command (this is one of the most basic commands in this bot). Paste it into the applicable place and edit the method name and arguments to fit your command. Implement the command in the body of your function. You can get more relevant info from the Context argument. If you need your commands to access some persistant state, you will need to edit the State struct with whatever function you need there. Take a look at the `serenity` and `poise` docs to learn more about the different Discord API interactions this framework covers. To register your command, you need the following things:
- in the `main.rs` file, in the `options` variable, add your command to the `commands` vec.
```rust
    // The commands to register for this bot
    commands: vec![
        help(),
        register(),
        general::voiceinfo(),
        general::echo(),
    ],
```
- re-compile and run the bot. type `~register` to re-register the commands in the guild.