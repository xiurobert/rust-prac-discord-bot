/*
 * This file is part of the rust-prac-discord-bot distribution
 * (https://github.com/xiurobert/rust-prac-discord-bot)
 * Copyright (c) 2022 Robert X
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;


#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        let bot_prefix: char = '!';
        if msg.content.len() >= 1 && char::from(msg.content.as_bytes()[0]) == bot_prefix {
            let command_and_args = &msg.content[1..];
            // println!("Text: {}", command_and_args);
            let info: Vec<&str> = command_and_args.split(' ').collect();
            if !info.is_empty() {
                let command = info[0];
                println!("User: ({}) <{}> called command: {}",
                         msg.author.tag(),
                         msg.author.id,
                         command);
                if command == "ping" {
                    let _ = msg.reply_ping(ctx,"Pong!").await;
                } else if command == "help" {
                    let _ = msg.reply_ping(ctx, concat!(
                    "Commands: \n",
                    "!ping - Pong!\n",
                    "**!help** - This message\n",
                    "!info - Information about the bot\n",
                    "!source - Source code\n",
                    "!store - Opens the in-game store\n",
                    "**Admin commands: **\n",
                    "!kick - Kick a user\n",
                    "!ban - Ban a user\n",
                    "!unban - Unban a user\n",
                    "!mute - Mute a user\n",
                    "!unmute - Unmute a user\n",
                    "!clear - Clear messages\n",
                    "!say - Make the bot say something\n",
                    "!eval - Evaluate code (Python)\n",
                    )).await;
                } else if command == "source" {
                    let _ = msg.channel_id.say(
                        ctx,
                        "https://github.com/xiurobert/rust-prac-discord-bot"
                    ).await;
                } else if command == "info" {
                    let _ = msg.channel_id.say(
                        ctx,
                        "Nothing here yet, lol".to_string()
                    ).await;
                } else {
                    let _ = msg.channel_id.say(
                        ctx,
                        "Command not found".to_string()
                    ).await;
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("
    rust-prac-discord-bot  Copyright (C) 2022  Robert
    This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.");
    // TODO: Implement 'show w' and 'show c'
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token).
        event_handler(Handler).await.expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}