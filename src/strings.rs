// TODO: Make these constants

pub fn help_string() -> &'static str {
    "Commands: \n,
    !ping - Pong!\n
    **!help** - This message\n
    !info - Information about the bot\n
    !source - Source code\n
    !store - Opens the in-game store\n
    **Admin commands: **\n
    !kick - Kick a user\n
    !ban - Ban a user\n
    !unban - Unban a user\n
    !mute - Mute a user\n
    !unmute - Unmute a user\n
    !clear - Clear messages\n
    !say - Make the bot say something\n
    !eval - Evaluate code (Python)\n"
}

pub fn not_implemented() -> &'static str {
    "Not implemented yet!"
}

pub fn github_url() -> &'static str {
    "https://github.com/xiurobert/rust-prac-discord-bot"
}

pub fn not_found() -> &'static str {
    "Not found!"
}

pub fn launch_text() -> &'static str {
    "
    rust-prac-discord-bot  Copyright (C) 2022  Robert
    This program comes with ABSOLUTELY NO WARRANTY; for details send `!about warranty'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `!about copying' for details.

    Note that all the above commands should be executed in discord.
    "
}