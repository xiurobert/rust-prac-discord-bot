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
mod strings;

use std::collections::{HashMap, HashSet};
use std::env;

use serenity::{async_trait, Client, model::{channel::Message, gateway::Ready}};
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::{Activity};
use serenity::model::guild::Role;
use serenity::model::id::RoleId;
use serenity::model::Permissions;
use serenity::model::user::OnlineStatus;
use crate::strings::{github_url, help_string, launch_text, not_found, not_implemented};


struct Handler;


#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        let bot_prefix: char = '!';
        if !msg.content.is_empty() && char::from(msg.content.as_bytes()[0]) == bot_prefix {
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
                    let _ = msg.reply_ping(ctx, help_string()).await;
                } else if command == "source" {
                    let _ = msg.channel_id.say(ctx, github_url()).await;
                } else if command == "info" {
                    let _ = msg.channel_id.say(ctx, not_implemented()).await;
                } else if command == "kick" {
                    match &msg.guild_id {
                        Some(guild_id) => {
                            let guild = ctx.cache
                                .guild(guild_id).await.unwrap();
                            let member_author = guild
                                .member(&ctx, msg.author.id)
                                .await.unwrap();

                            /*
                            Here, I check for all the roles in the server that contain the kick
                            permission and store it into a HashMap.
                            The reason for using a hashmap is it's O(1) lookup time.
                            This is much better than just iterating every single role

                            TL;DR: This bit filter out the roles that have a kick permission
                             */
                            let mut roles_that_can_kick =
                                HashMap::<&RoleId, &Role>::new();
                            for (r_id, r) in guild.roles.iter() {
                                if r.has_permission(Permissions::KICK_MEMBERS) {
                                    roles_that_can_kick.insert(r_id, r);
                                }
                            }

                            // Now we just check that the member actually has a role that can kick.
                            // We look it up in the hashmap.
                            // This runs in O(n) linear time (n = number of roles that the member has)
                            // Probably does not matter for a discord bot, but hey.

                            for role in member_author.roles {
                                if roles_that_can_kick.contains_key(&role) {
                                    let member_to_kick = guild
                                        .member(&ctx, info[1].parse::<u64>().unwrap())
                                        .await.unwrap();
                                    let _ = guild.kick(&ctx, &member_to_kick).await;
                                    let _ = msg.channel_id.say(
                                        ctx,
                                        format!("Kicked {}", &member_to_kick.user.name)
                                    ).await;
                                    break;
                                }
                            }
                        },
                        None => {
                            let _ = &msg.reply_ping(&ctx,
                                                    "You need to be in a guild t\
                                                    o use this command.").await;
                        }
                    }

                } else {
                    let _ = msg.channel_id.say(ctx, not_found()).await;
                }
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let activity = Activity::listening("!help");
        ctx.set_presence(Some(activity), OnlineStatus::Online).await;
    }
}

#[tokio::main]
async fn main() {
    println!(launch_text());
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token).
        event_handler(Handler).await.expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}